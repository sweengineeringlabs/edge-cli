# Architecture — edge-cli

## Sequence

> The CLI loads `Config` from TOML, runs `ValidationProvider` over an input path, optionally scaffolds curl test files, and reports results.

```mermaid
sequenceDiagram
    participant Main
    participant ConfigLoader
    participant ValidationProvider
    participant CurlScaffolder
    participant FileSystem

    Main->>ConfigLoader: ApplicationConfigBuilder::build()
    ConfigLoader-->>Main: Config{target_path, output_dir, …}

    Main->>ValidationProvider: provide_validation(target_path)
    ValidationProvider->>FileSystem: read input files
    FileSystem-->>ValidationProvider: raw content
    ValidationProvider->>ValidationProvider: run validation rules
    ValidationProvider-->>Main: ValidationReport{pass/fail, Vec<Issue>}

    alt scaffold requested
        Main->>CurlScaffolder: scaffold(ScaffoldSpec, output_dir)
        CurlScaffolder->>FileSystem: write curl test files
        CurlScaffolder-->>Main: ScaffoldReport{files_written: Vec<PathBuf>}
    end

    Main->>Main: print report
    Main-->>OS: exit(0) or exit(1)
```

## Data Flow

> A `Config` + filesystem path enter the CLI pipeline; a `ValidationReport` (and optional `ScaffoldReport`) exit.

```mermaid
flowchart LR
    A["Config\n───────────\ntarget_path: PathBuf\noutput_dir: PathBuf\nrules: Vec<String>"] --> B["ValidationProvider\n::provide_validation\n(target_path)"]

    B --> C["read input files\nfrom FileSystem"]
    C --> D["apply validation rules\n(schema, contract, naming)"]
    D -->|all pass| E["ValidationReport\n{status: Pass\nissues: []}"]
    D -->|failures| F["ValidationReport\n{status: Fail\nissues: Vec<Issue>}"]

    A --> G{scaffold?}
    G -->|yes| H["ScaffoldSpec\n(derived from Config)"]
    H --> I["CurlScaffolder\n::scaffold(spec, output_dir)"]
    I --> J["ScaffoldReport\n{files_written:\nVec<PathBuf>}"]

    E --> K["exit(0)"]
    F --> L["exit(1)"]
```
