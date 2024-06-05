use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Tracer;
use opentelemetry_sdk::{runtime, Resource};
use tonic::metadata::MetadataMap;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use helpers::MockWriter;

pub fn init_test_subscriber() -> MockWriter {
    let writer = MockWriter::new();

    let tracer = init_tracer();
    let otel = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(LevelFilter::INFO)
        .with(otel)
        .with(
            fmt::layer()
                .json()
                .with_span_events(FmtSpan::NEW | FmtSpan::EXIT)
                .without_time()
                .with_target(false)
                .flatten_event(true)
                .with_writer({
                    let writer = writer.clone();
                    move || writer.clone()
                }),
        )
        .init();

    writer
}

pub fn init_tracer() -> Tracer {
    let honeycomb_key =
        std::env::var("HONEYCOMB_API_KEY").expect("`HONEYCOMB_API_KEY` must be set");
    let mut map = MetadataMap::with_capacity(1);
    map.insert("x-honeycomb-team", honeycomb_key.try_into().unwrap());

    // Correctly configuring your exporter is a bit of a black art and highly-dependent on the
    // specifics of your deployment environment.
    // We won't go into the details here, but you can read more about it in the OpenTelemetry
    // documentation (or grab me after the workshop to talk about it).
    // At a super high-level: you want batching and you want a sensible sampling strategy,
    // but beyond that it's hard to give general advice.
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
            Resource::new(vec![KeyValue::new(
                "service.name",
                "rust-telemetry-workshop",
            )]),
        ))
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("https://api.honeycomb.io/api/traces")
                .with_timeout(std::time::Duration::from_secs(5))
                .with_metadata(map),
        )
        .install_batch(runtime::Tokio)
        .unwrap()
}
