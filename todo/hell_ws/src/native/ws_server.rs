// [git](https://github.com/snapview/tokio-tungstenite)
// [docs](https://docs.rs/tokio-tungstenite/0.17.2/tokio_tungstenite/)



use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use futures::{StreamExt, TryStreamExt, future, pin_mut, SinkExt};
use hell_error::{HellResult, ErrToHellErr};
use tokio::net::{TcpStream, TcpListener};
use futures_channel::mpsc::{unbounded, UnboundedSender}; // TODO: why not tokio?

use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;



type TokioMutex<T> = tokio::sync::Mutex<T>;
type Tx = UnboundedSender<Message>;
type PeerMap = Arc<TokioMutex<HashMap<SocketAddr, Tx>>>;



pub struct WSServer {
    addr: String,
    peers: PeerMap,
}

impl WSServer {
    pub fn new(addr: impl Into<String>) -> WSServerHandle {
        let addr: String = addr.into();
        let peers = PeerMap::new(TokioMutex::new(HashMap::new()));

        let server = Self {
            addr,
            peers,
        };

        let handle = WSServerHandle {
            server: Arc::new(server),
        };

        handle
    }
}


impl WSServer {
    pub async fn start_listening(&self) -> HellResult<()> {
        println!("listenting on: {}", &self.addr);
        let listener = TcpListener::bind(&self.addr).await?;

        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(
                Self::handle_connection(
                    self.peers.clone(),
                    stream,
                    addr
                )
            );
        }

        println!("stop listening on: {} ", &self.addr);

        Ok(())
    }

    pub async fn send_msg_to_all(&self, msg: impl Into<String>) -> HellResult<()> {
        let msg: String = msg.into();
        println!("send msg: {}", msg);

        let mut peers = self.peers.lock().await;
        // let mut peers = self.peers.lock().unwrap();

        for (_, tx) in peers.iter_mut() {
            tx.send( Message::Text( msg.clone() ) ).await.to_hell_err(hell_error::HellErrorKind::GenericError)?;
        }

        Ok(())
    }
}

impl WSServer {

    async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) -> HellResult<()> {
        println!("incoming TCP connection from: {}", addr);

        let ws_stream: WebSocketStream<_> = tokio_tungstenite::accept_async(raw_stream).await?;
        let (tx, rx) = unbounded();
        peer_map
            .lock()
            .await
            .insert(addr, tx);

        let (outgoing, incoming) = ws_stream.split();

        let broadcast_incoming = incoming.try_for_each(|msg| {
            println!("received a message from {}: {}", addr, msg.to_text().unwrap());

            // HACK: async closures aren't stable yet
            tokio::task::block_in_place(|| {
                let peers = peer_map.blocking_lock();
                // let peers = peer_map.lock().unwrap();

                let broadcast_recipients = peers.iter()
                    .filter(|(peer_addr, _)| peer_addr != &&addr)
                    .map(|(_, ws_sink)| ws_sink);

                for recp in broadcast_recipients {
                    recp.unbounded_send(msg.clone()).unwrap();
                }
            });

            future::ok(())
        });

        let receive_from_others = rx.map(Ok).forward(outgoing);
        pin_mut!(broadcast_incoming, receive_from_others);
        future::select(broadcast_incoming, receive_from_others).await;

        println!("{} disconnected", &addr);
        peer_map
            .lock()
            .await
            .remove(&addr);

        Ok(())
    }
}

// ----------------------------------------------

pub struct WSServerHandle {
    server: Arc<WSServer>,
}

impl Clone for WSServerHandle {
    fn clone(&self) -> Self {
        Self { server: self.server.clone() }
    }
}

impl WSServerHandle {
    pub async fn start_listening(self) -> HellResult<()> {
        self.server.start_listening().await
    }

    pub async fn send_msg(&self, msg: impl Into<String>) -> HellResult<()> {
        self.server.send_msg_to_all(msg).await
    }
}
