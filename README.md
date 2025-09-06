# cargo-test-json-2-html

Convert cargo test JSON output to beautiful, self-contained HTML reports.

## Features

- 🎨 **Beautiful HTML reports** - Clean, modern design with responsive layout
- 📊 **Test statistics** - Pass/fail/ignored counts
- 🔍 **Detailed test output** - Collapsible stdout/stderr for each test
- 🛡️ **Robust error handling** - Gracefully handles mixed JSON/non-JSON output
- 📱 **Self-contained** - Single HTML file with inline CSS and JavaScript
- 🔗 **Source linking** - Configurable source code links

**[📋 Example Report](examples/test-report.html)**

## Installation

Most users will probably want to integrate this as a library into an existing test runner system. However, absent of that, it is possible to run it as a CLI as well:

```bash
cargo install --locked cargo-test-json-2-html
```

## Usage

### Basic Usage

Generate JSON test output and convert to HTML:

```bash
# Generate test report
cargo +nightly test -- --format json -Z unstable-options --show-output > test-output.json 2>&1
cargo-test-json-2-html -i test-output.json -o report.html
```

### Pipeline Usage

```bash
# Direct pipeline (requires nightly Rust)
cargo +nightly test -- --format json -Z unstable-options --show-output 2>&1 | cargo-test-json-2-html -o report.html
```

### Library Usage

```rust
use cargo_test_json_2_html::{convert_to_html, Config, SourceLinker};

// Basic usage
let json_output = r#"{ "type": "test", "name": "my_test", "event": "ok" }"#;
let config = Config::default();
let html = convert_to_html(json_output, config);

// With custom source linking
struct GitHubLinker {
    repo: String,
}

impl SourceLinker for GitHubLinker {
    fn link(&self, file: &str, line: u32) -> Option<String> {
        Some(format!("https://github.com/{}/blob/main/{}#L{}", self.repo, file, line))
    }
}

let config = Config::builder()
    .source_linker(Box::new(GitHubLinker {
        repo: "user/repo".to_string()
    }))
    .build();
let html = convert_to_html(json_output, config);
```

## CLI Options

```
cargo-test-json-2-html [OPTIONS]

Options:
  -i, --input <INPUT>    Input file (use - for stdin) [default: -]
  -o, --output <OUTPUT>  Output file (use - for stdout) [default: -]
  -h, --help             Print help
```

## Examples

### Generate Report for Current Project

```bash
# Run tests and generate HTML report
cargo +nightly test -- --format json -Z unstable-options --show-output > test-results.json 2>&1
cargo-test-json-2-html -i test-results.json -o test-report.html
open test-report.html  # macOS
# or
xdg-open test-report.html  # Linux
```

### CI/CD Integration

```yaml
# GitHub Actions example
- name: Run tests and generate report
  run: |
    cargo +nightly test -- --format json -Z unstable-options --show-output > test-results.json 2>&1 || true
    cargo-test-json-2-html -i test-results.json -o test-report.html

- name: Upload test report
  uses: actions/upload-artifact@v3
  with:
    name: test-report
    path: test-report.html
```

## Error Handling

The tool gracefully handles various input scenarios:

- **Mixed output**: Compilation messages mixed with JSON test results
- **Invalid JSON**: Malformed JSON lines are reported as errors
- **Empty input**: Generates a valid report even with no tests
- **Parsing errors**: Full error context included in the HTML report

## Requirements

- **For JSON output**: Rust nightly (for `--format json -Z unstable-options`)
- **For the tool**: Rust stable (1.70+)

## Output Format

The generated HTML report includes:

- **Summary statistics** with pass/fail/ignored counts
- **Failed tests section** with full error output
- **Passed tests section** with stdout/stderr (collapsible)
- **Ignored tests section**
- **Parsing errors section** (if any)
- **Raw output section** for non-JSON lines

## Contributing

Contributions welcome! Please check the [issues](https://github.com/user/cargo-test-json-2-html/issues) for planned features.

### Development and Debugging

To generate a test report for debugging the HTML output:

```bash
# Generate test output from this project
cargo +nightly test -- --format json -Z unstable-options --show-output > test-output.json 2>&1

# Convert to HTML report
cargo run -- -i test-output.json -o test-report.html

# Open the report
open test-report.html  # macOS
# or
xdg-open test-report.html  # Linux
```

This will create a report showing the actual test results from this project, including any failures, which is useful for testing the HTML generation and styling.

## License

MIT OR Apache-2.0
