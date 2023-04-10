use hell_error::{HellResult, ErrToHellErr};

use futures::{StreamExt, pin_mut, future};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;





pub struct WSClient {
    url: Url,
}

impl WSClient {
    pub fn new(addr: &str) -> HellResult<Self> {
        let url = Url::parse(addr).to_hell_err(hell_error::HellErrorKind::GenericError)?;

        Ok(Self {
            url,
        })
    }

    pub async fn start_running(self) -> HellResult<()> {
        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        tokio::spawn(read_stdin(stdin_tx));

        let (ws_stream, _) = connect_async(self.url.clone()).await.unwrap();
        println!("WebSocket handshake has been successfully complted");

        let (write, read) = ws_stream.split();

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|msg| async {
                let data = msg.unwrap().into_data();
                tokio::io::stdout().write_all(&data).await.unwrap();
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;

        Ok(())
    }
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) -> HellResult<()>{
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };

        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).to_hell_err(hell_error::HellErrorKind::GenericError)?;
    }

    Ok(())
}
