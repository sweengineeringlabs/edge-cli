//! Example: validate a directory using BinSvc.

fn main() {
    let svc = swe_edge_bin::BinSvc::new();
    let config = swe_edge_bin::Config::new().with_command(swe_edge_bin::Command::Validate {
        path: std::path::PathBuf::from("."),
    });
    match svc.execute(&config) {
        Ok(()) => println!("Validation passed"),
        Err(e) => eprintln!("Validation failed: {e}"),
    }
}
