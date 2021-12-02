//! This is the client for the maths service, it is a command line
//! utility and only responsible for making a call to the
//! `maths-frontend-service` endpoints for `add` and `mul`.

use maths_common::proto::maths::{maths_client::MathsClient, AddRequest, MulRequest};
use maths_common::trace_req;
use opentelemetry::global;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use structopt::StructOpt;
use tracing::*;
use tracing_subscriber::prelude::*;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case", global_setting = structopt::clap::AppSettings::ColoredHelp)]
enum Opts {
    #[structopt(name = "add")]
    /// Add two numbers together.
    Add {
        #[structopt()]
        val1: i32,
        #[structopt()]
        val2: i32,
    },

    #[structopt(name = "mul")]
    /// Multiply two numbers together.
    Mul {
        #[structopt()]
        val1: i32,
        #[structopt()]
        val2: i32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Jaeger and OpenTelemetry are setup slightly different in the
    // client than they are in the servers. Here we use the
    // `install_simple()` function, which will send the metrics on
    // drop.
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("maths-client")
        .install_simple()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()?;

    match Opts::from_args() {
        Opts::Add { val1, val2 } => add(val1, val2).await?,
        Opts::Mul { val1, val2 } => mul(val1, val2).await?,
    };

    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}

#[instrument]
async fn add(val1: i32, val2: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MathsClient::connect("http://127.0.0.1:12340")
        // We instrument the rpc calls with an info span, though this
        // is not required.
        .instrument(info_span!("rpc: frontend client connect"))
        .await?;

    let req = trace_req(AddRequest { val1, val2 });
    let resp = client
        .add(req)
        .instrument(info_span!("rpc: call frontend add"))
        .await?
        .into_inner();
    println!("{} + {} = {}", val1, val2, resp.result);
    Ok(())
}

#[instrument]
async fn mul(val1: i32, val2: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MathsClient::connect("http://127.0.0.1:12340")
        .instrument(info_span!("rpc: frontend client connect"))
        .await?;

    let req = trace_req(MulRequest { val1, val2 });
    let resp = client
        .mul(req)
        .instrument(info_span!("rpc: call frontend mul"))
        .await?
        .into_inner();
    println!("{} * {} = {}", val1, val2, resp.result);
    Ok(())
}
