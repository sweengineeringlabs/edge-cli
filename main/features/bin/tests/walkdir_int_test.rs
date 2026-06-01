//! Integration tests exercising the walkdir dependency.

use std::fs;
use swe_edge_bin::BinSvc;
use walkdir::WalkDir;

/// @covers: ConfigValidator::validate (walkdir)
#[test]
fn test_walkdir_traverses_nested_directories() {
    let temp = tempfile::TempDir::new().unwrap();

    // Create nested structure
    let nested = temp.path().join("subdir").join("deep");
    fs::create_dir_all(&nested).unwrap();
    fs::write(nested.join("config.toml"), "[section]\nkey = \"val\"").unwrap();
    fs::write(temp.path().join("root.toml"), "[root]\nkey = \"val\"").unwrap();

    // Direct walkdir usage to confirm it traverses nested dirs
    let toml_count = WalkDir::new(temp.path())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|n| n.ends_with(".toml"))
                .unwrap_or(false)
        })
        .count();
    assert_eq!(toml_count, 2);

    let svc = BinSvc::new();
    let validator = svc.config_validator();
    let report = validator.validate(temp.path()).unwrap();

    // walkdir should find both .toml files
    assert!(report.passed.len() >= 2);
}

/// @covers: DomainValidator::validate (walkdir)
#[test]
fn test_walkdir_finds_architecture_toml() {
    let temp = tempfile::TempDir::new().unwrap();
    let workspace = temp.path().join("handler-workspace");
    fs::create_dir_all(&workspace).unwrap();

    let arch_content = r#"
[package.metadata.struct-engine]
service_type = "handler"
"#;
    fs::write(workspace.join("architecture.toml"), arch_content).unwrap();
    fs::create_dir_all(workspace.join("src").join("api")).unwrap();
    fs::create_dir_all(workspace.join("src").join("saf")).unwrap();

    // Direct walkdir usage
    let arch_count = WalkDir::new(temp.path())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "architecture.toml")
        .count();
    assert_eq!(arch_count, 1);

    let svc = BinSvc::new();
    let validator = svc.domain_validator();
    let report = validator.validate(temp.path()).unwrap();

    assert!(report
        .passed
        .iter()
        .any(|p| p.contains("api/") || p.contains("saf/")));
}
