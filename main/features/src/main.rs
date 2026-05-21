//! edge CLI — domain and config validator, curl test scaffolder.

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::io::stdout)
        .init();

    if let Err(e) = swe_edge_cli_features::run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
