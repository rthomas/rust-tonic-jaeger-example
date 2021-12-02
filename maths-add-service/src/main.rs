use maths_common::proto::add::add_server::AddServer;
use tonic::transport::Server;

mod add;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    maths_common::otel_jaeger_init("maths-add-service")?;
    Server::builder()
        .add_service(AddServer::new(add::AddService))
        .serve("127.0.0.1:12341".parse()?)
        .await?;

    Ok(())
}
