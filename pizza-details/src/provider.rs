use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::sdk::Resource;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;

pub fn setup_provider() {
    let endpoint_url = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .expect("could not find otlp exporter endpoint");
    global::set_text_map_propagator(TraceContextPropagator::new());
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(endpoint_url);
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry::sdk::trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "pizza-details",
            )])),
        )
        .with_exporter(exporter)
        .install_simple()
        .expect("could not init the tracer");
}

pub fn shutdown_provider() {
    shutdown_tracer_provider();
}
