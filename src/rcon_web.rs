use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

use crate::rcon::{Rcon, RconMessage};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRconMessage {
    pub Message: String,
    pub Identifier: i32,
    pub Type: String,
    pub Stacktrace: Option<String>,
}

#[derive(Debug)]
pub struct WebRcon {
    pub receiver: Receiver<RconMessage>,
    write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}

#[async_trait]
impl Rcon for WebRcon {
    async fn connect(ip: &str, port: &str, password: &str) -> Self {
        let url = url::Url::parse(&format!("ws://{}:{}/{}", ip, port, password)).unwrap();

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (write, read) = ws_stream.split();

        let (sender, receiver) = async_channel::unbounded();
        tokio::spawn(WebRcon::receive_data(read, sender));

        Self {
            receiver,
            write_stream: write,
        }
    }

    async fn execute(&mut self, command: &str) {
        let message = WebRconMessage {
            Message: command.to_string(),
            Identifier: 10,
            Type: String::default(),
            Stacktrace: None,
        };

        self.write_stream
            .send(serde_json::to_string(&message).unwrap().into())
            .await
            .unwrap();
    }
}

impl WebRcon {
    async fn receive_data(
        stream_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        sender: Sender<RconMessage>,
    ) {
        stream_read
            .for_each(|message| async {
                if let Ok(message) = message {
                    if let Ok(data) = message.into_text() {
                        let message: WebRconMessage = serde_json::from_str(&data).unwrap();

                        sender
                            .send(RconMessage::from_webrcon_message(message))
                            .await
                            .unwrap();
                    } else {
                        sender.send(RconMessage::Disconnected).await.unwrap();
                    }
                } else {
                    sender.send(RconMessage::Disconnected).await.unwrap();
                }
            })
            .await;
    }
}
