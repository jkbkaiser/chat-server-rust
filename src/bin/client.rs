use chat_server::client::Client;

#[tokio::main]
async fn main() {
    // User url lib + some add cli

    let client = Client::new("ws://0.0.0.0:8080");
    client.run().await;
}
