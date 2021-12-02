use opentelemetry::{
    global,
    propagation::{Extractor, Injector},
    sdk::propagation::TraceContextPropagator,
};
use tonic::{service::Interceptor, Request, Status};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::prelude::*;

/// Here we expose the protos that we've created for our services -
/// nothing special here.
pub mod proto {
    pub mod maths {
        tonic::include_proto!("maths");
    }

    pub mod add {
        tonic::include_proto!("add");
    }

    pub mod mul {
        tonic::include_proto!("mul");
    }
}

/// This will init OpenTelemetry and Jaeger, registering the
/// tracer. This differs from the `maths-client` binary as we use the
/// `install_batch` function which will queue the trace tx, rather
/// than sending it on drop.
pub fn otel_jaeger_init(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .install_batch(opentelemetry::runtime::Tokio)?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()?;

    Ok(())
}

/// In order for our trace to be propagated between services, we need
/// to inject the trace context into the request - this function just
/// takes a request body (one of the proto messages we defined) and
/// creates a new `tonic::Request`, injects the context and returns
/// that request.
pub fn trace_req<T>(req: T) -> Request<T> {
    let mut req = Request::new(req);

    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(
            &tracing::Span::current().context(),
            &mut MutMetadataMap(req.metadata_mut()),
        )
    });

    req
}

// #[derive(Clone, Debug)]
// pub struct TraceInterceptor;

// impl Interceptor for TraceInterceptor {
//     fn call(&mut self, req: Request<()>) -> Result<Request<()>, Status> {
//         eprintln!("interceptor: {:?}", req);
//         let parent_cx =
//             global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(req.metadata())));
// 	parent_cx.attach()
//         tracing::info_span!(parent: &parent_cx, "trace interceptor");
//         tracing::Span::current().set_parent(parent_cx);
//         Ok(req)
//     }
// }

/// This is the other end of the `trace_req` function - when we
/// receive a `tonic::Request` we need to extract the context and set
/// it as our current. This can just be called in your rpc handler.
pub fn set_trace_ctx<T>(req: &Request<T>) {
    let parent_cx =
        global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(req.metadata())));
    tracing::Span::current().set_parent(parent_cx);
}

struct MetadataMap<'a>(pub &'a tonic::metadata::MetadataMap);
struct MutMetadataMap<'a>(pub &'a mut tonic::metadata::MetadataMap);

impl<'a> Injector for MutMetadataMap<'a> {
    /// Set a key and value in the MetadataMap.  Does nothing if the key or value are not valid inputs
    fn set(&mut self, key: &str, value: String) {
        if let Ok(key) = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()) {
            if let Ok(val) = tonic::metadata::MetadataValue::from_str(&value) {
                self.0.insert(key, val);
            }
        }
    }
}

impl<'a> Extractor for MetadataMap<'a> {
    /// Get a value for a key from the MetadataMap.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}
