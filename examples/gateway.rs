use async_federation::{gateway::Gateway, source::Remote};

#[tokio::main]
async fn main() {
    let mut server = Gateway::new(vec![
        Box::new(Remote::new("accounts", "http://localhost:4001")),
        Box::new(Remote::new("products", "http://localhost:4002")),
        Box::new(Remote::new("reviews", "http://localhost:4003")),
    ])
    .await;
    server.serve().await.expect("Error when serving");
}
