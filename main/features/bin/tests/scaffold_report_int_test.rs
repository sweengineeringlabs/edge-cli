//! Integration tests for ScaffoldReport.

use std::path::PathBuf;
use swe_edge_bin::ScaffoldReport;

/// @covers: ScaffoldReport::new
#[test]
fn test_scaffold_report_new_is_empty() {
    let report = ScaffoldReport::new();
    assert!(report.files_written.is_empty());
}

/// @covers: ScaffoldReport::add_file
#[test]
fn test_scaffold_report_add_file() {
    let mut report = ScaffoldReport::new();
    let path = PathBuf::from("/tmp/test.sh");
    report.add_file(path.clone());
    assert_eq!(report.files_written.len(), 1);
    assert_eq!(report.files_written[0], path);
}
