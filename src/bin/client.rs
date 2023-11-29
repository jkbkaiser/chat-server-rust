use chat_server::client::FrontEnd;
// use chat_server::client::messaging::UserInput;
// use futures_util::SinkExt;
// use std::error;
// use std::io;
// use std::io::Write;
// use tokio::{self, net::TcpStream};
// use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
//
// type MyWebSocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[tokio::main]
async fn main() {
    // User url lib + some add cli
    let front_end = FrontEnd::new("ws://0.0.0.0:8080");
    front_end.run().await;
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn error::Error>> {
//     println!("Starting client ...");
//     println!("Connecting to server ...");
//
//     match connect_async("ws://0.0.0.0:8080").await {
//         Ok((ws_stream, _)) => {
//             println!("Connected to server");
//             run(Handler::new(ws_stream)).await?;
//         }
//         Err(err) => {
//             println!("Failed to connect to server {err:?}");
//         }
//     }
//
//     Ok(())
// }
//
// async fn run(mut handler: Handler) -> Result<(), Box<dyn error::Error>> {
//     let mut out = io::stdout();
//     write!(out, ">> ")?;
//     out.flush()?;
//
//     for line in io::stdin().lines() {
//         write!(out, ">> ")?;
//         out.flush()?;
//
//         match line {
//             Ok(line) => {
//                 let message = UserInput::from_line(line)?;
//                 handler.handle_user_input(message).await?;
//             }
//             Err(_) => {
//                 println!("Could not read line");
//             }
//         }
//     }
//
//     Ok(())
// }
//
// struct Handler {
//     ws_stream: MyWebSocketStream,
// }
//
// impl Handler {
//     fn new(ws_stream: MyWebSocketStream) -> Self {
//         Handler { ws_stream }
//     }
//
//     async fn handle_user_input(
//         &mut self,
//         user_intput: UserInput,
//     ) -> Result<(), Box<dyn error::Error>> {
//         // println!("parsed input: {message:?}");
//
//         self.ws_stream
//             .send(Message::Binary(user_intput.to_bin()?))
//             .await
//             .unwrap();
//
//         // println!("sent message");
//         Ok(())
//     }
// }
