use maths_common::proto::maths::maths_server::MathsServer;
use tonic::transport::Server;

mod frontend;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    maths_common::otel_jaeger_init("maths-frontend")?;
    // let maths_service = MathsServer::with_interceptor(
    //     frontend::MathsService::new().await?,
    //     maths_common::TraceInterceptor,
    // );

    let maths_service = MathsServer::new(frontend::MathsService::new().await?);

    Server::builder()
        .add_service(maths_service)
        .serve("127.0.0.1:12340".parse()?)
        .await?;

    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}
