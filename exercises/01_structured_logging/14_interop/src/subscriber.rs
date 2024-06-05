use helpers::MockWriter;
use tracing_log::LogTracer;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_test_subscriber() -> MockWriter {
    let writer = MockWriter::new();
    let writer2 = writer.clone();

    tracing_subscriber::fmt()
        .with_writer(move || writer.clone())
        .with_span_events(FmtSpan::FULL)
        .compact()
        .init();

    LogTracer::init().unwrap();

    writer2
}
