// [git](https://github.com/snapview/tokio-tungstenite)
// [docs](https://docs.rs/tokio-tungstenite/0.17.2/tokio_tungstenite/)



use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Mutex, Arc};

use futures::{StreamExt, TryStreamExt, future, pin_mut};
use hell_error::HellResult;
use tokio::net::{TcpStream, TcpListener};
// use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use futures_channel::mpsc::{unbounded, UnboundedSender}; // TODO: why not tokio?

use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;



type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;



pub struct WSServer {
    addr: String,
    peers: PeerMap,
}

impl WSServer {
    pub fn new(addr: String) -> Self {
        let peers = PeerMap::new(Mutex::new(HashMap::new()));

        Self {
            addr,
            peers,
        }
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

        Ok(())
    }
}

impl WSServer {
    async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) -> HellResult<()> {
        println!("incoming TCP connection from: {}", addr);

        let ws_stream: WebSocketStream<_> = tokio_tungstenite::accept_async(raw_stream).await?;
        let (tx, rx) = unbounded();
        peer_map.lock().unwrap().insert(addr, tx);

        let (outgoing, incoming) = ws_stream.split();

        let broadcast_incoming = incoming.try_for_each(|msg| {
            println!("received a message from {}: {}", addr, msg.to_text().unwrap());

            let peers = peer_map.lock().unwrap();

            let broadcast_recipients = peers.iter()
                .filter(|(peer_addr, _)| peer_addr != &&addr)
                .map(|(_, ws_sink)| ws_sink);

            for recp in broadcast_recipients {
                recp.unbounded_send(msg.clone()).unwrap();
            }

            future::ok(())
        });

        let receive_from_others = rx.map(Ok).forward(outgoing);
        pin_mut!(broadcast_incoming, receive_from_others);
        future::select(broadcast_incoming, receive_from_others).await;

        println!("{} disconnected", &addr);
        peer_map.lock().unwrap().remove(&addr);

        Ok(())
    }
}
