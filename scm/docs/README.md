# swe-edge-bin Overview

## WHAT

`swe-edge-bin` is the edge CLI — a developer tool for validating TOML configuration and domain handler contracts, scaffolding curl test suites, and walking workspaces.

| Capability | Type | Description |
|------------|------|-------------|
| `Command` | value object | Parsed CLI subcommand (`Validate`, `Scaffold`) with typed arguments |
| `ValidationReport` / `ValidationFailure` | value objects | Structured output from validation passes; `is_clean()` returns `true` when no failures |
| `ValidationProvider` | trait | Pluggable validation strategy — implement to add custom rule sets |
| `NoopValidationProvider` | type | Default no-op provider for testing and dry-run mode |
| `ProviderRegistry` / `ProviderRegistryContract` | types | Registrar that collects all active `ValidationProvider` impls |
| `ConfigValidator` / `ConfigValidatorContract` | types | Validates TOML config files against expected sections and required keys |
| `DomainValidator` / `DomainValidatorContract` | types | Validates domain handler stubs against SEA structural rules |
| `Scaffolder` / `CurlScaffolder` / `CurlScaffolderContract` | types | Generates curl test scripts from `HandlerSpec` descriptors |
| `HandlerSpec` / `ScaffoldSpec` / `ScaffoldReport` | value objects | Input/output shapes for the scaffolding pipeline |
| `BinError` | error type | Unified CLI error — wraps config, validation, I/O, and scaffolding failures |
| `BinSvc` | factory | Top-level entry point — wires command dispatch, validation pipeline, and scaffolding |
| `ApplicationConfigBuilder` | builder | Composes TOML config layers (default → workspace override → env) for CLI context |

## WHY

| Problem | Solution |
|---------|----------|
| Developers misconfigure TOML sections (wrong keys, missing required fields) and only discover errors at runtime | `ConfigValidator` catches structural TOML violations before `cargo run` by walking all config layers offline |
| SEA structural rules (api/ core/ saf/ spi/ layout, naming conventions, unused re-exports) are manual to enforce | `DomainValidator` walks workspace source trees and reports rule violations with file + line references |
| Writing curl test suites by hand for every new handler is repetitive and error-prone | `CurlScaffolder` generates ready-to-run curl scripts from `HandlerSpec` descriptors in one command |
| Validation logic is not pluggable — adding a new rule means modifying core validator code | `ValidationProvider` trait + `ProviderRegistry` let downstream teams register custom rules without touching core |
| CLI errors from different subsystems (config, I/O, validation, scaffolding) need uniform error reporting | `BinError` bridges all failure modes into a single enum with actionable messages |
