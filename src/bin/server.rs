use chat_server::server::Server;

#[tokio::main]
async fn main() {
    // User url lib + some add cli

    let server = Server::new("0.0.0.0:8080");
    server.run().await;
}
