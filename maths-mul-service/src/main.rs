use maths_common::proto::mul::mul_server::MulServer;
use tonic::transport::Server;

mod mul;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    maths_common::otel_jaeger_init("maths-mul-service")?;

    Server::builder()
        .add_service(MulServer::new(mul::MulService::new().await?))
        .serve("127.0.0.1:12342".parse()?)
        .await?;

    Ok(())
}
