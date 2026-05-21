//! edge CLI — domain and config validator, curl test scaffolder.

fn main() {
    if let Err(e) = swe_edge_cli::run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
