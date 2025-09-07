# Cargo Test JSON to HTML Report Generator - Tasks

## Research Summary

### Cargo Test JSON Output Format
- Cargo supports `--format json` with nightly features: `cargo +nightly test -- --format json -Z unstable-options --show-output`
- JSON output includes:
  - Test events: `started`, `ok`, `failed`, `ignored`
  - Test names and execution times
  - stdout/stderr output (combined in `stdout` field)
  - Suite summary with pass/fail counts

### Template Engine Options
Based on research, **Handlebars** is the industry standard choice:
- Widely used at Amazon (Sonar, RTN, various internal tools)
- Mature ecosystem with good Rust support (`handlebars` crate)
- Logic-less templates with helpers for safety
- Good performance and maintainability

### ANSI Color Conversion
- Need to parse ANSI escape sequences in stdout/stderr
- Convert to HTML `<span>` tags with CSS classes
- Use existing crate like `ansi-to-html` or `console`

## Implementation Tasks

### 1. Core Library Structure
- [ ] Create `lib.rs` with main conversion function accepting JSON string
- [ ] Define data structures for parsed JSON test results
- [ ] Implement JSON parsing with `serde` and graceful error handling
- [ ] Create HTML template with Handlebars
- [ ] Handle non-JSON lines gracefully (skip or include as raw text)

### 2. JSON Parsing & Error Handling
- [ ] Parse cargo test JSON output line by line
- [ ] Skip non-JSON lines gracefully (compilation output, etc.)
- [ ] Handle different event types (`suite`, `test`)
- [ ] Extract test metadata (name, status, duration, output)
- [ ] Group tests by status (passed, failed, ignored)
- [ ] Collect parsing errors and include in HTML output for debugging

### 3. HTML Template
- [ ] Create Handlebars template for test report
- [ ] Include test summary statistics
- [ ] Display individual test results with collapsible details
- [ ] Show stdout/stderr in `<pre>` tags (raw for MVP)
- [ ] Add inline CSS for styling and responsive design (self-contained)
- [ ] Include error section with full context for debugging

### 4. ANSI Color Conversion (Post-MVP)
- [ ] Defer ANSI handling until after MVP
- [ ] For now, display raw stdout/stderr as-is in `<pre>` tags
- [ ] Future: Use existing crate like `ansi-to-html` if simple

### 5. Source Code Linking
- [ ] Accept user-provided closure for generating source links
- [ ] Parse file paths from test output/panic messages
- [ ] Generate clickable links using user's template function

### 6. Testing with Insta
- [ ] Create comprehensive test module with various scenarios:
  - Passing tests with stdout
  - Failing tests with stderr and panic info
  - Ignored tests
  - Tests with ANSI colors
  - Mixed JSON and non-JSON lines
  - Invalid JSON lines
  - Empty input
  - Malformed test events
- [ ] Use `insta` for snapshot testing HTML output
- [ ] Generate sample JSON output for all test scenarios
- [ ] Test error handling and graceful degradation

### 7. CLI Tool
- [ ] Create `main.rs` for command-line interface
- [ ] Accept JSON input from stdin or file
- [ ] Output HTML to stdout or file
- [ ] Add configuration options (source URL, CSS theme)

### 8. Documentation
- [ ] Write comprehensive README.md with usage examples
- [ ] Document configuration options
- [ ] Include sample output screenshots
- [ ] Add library API documentation

## Dependencies
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
handlebars = "4.0"
bon = "2.0"  # for builder pattern on Config
clap = "4.0"  # for CLI

[dev-dependencies]
insta = "1.0"  # for snapshot testing
tempfile = "3.0"  # for integration tests
```

## File Structure
```
src/
├── lib.rs              # Main library code
├── main.rs             # CLI tool
├── parser.rs           # JSON parsing logic
├── template.rs         # HTML template handling
└── test_data.rs        # Test data structures

templates/
└── report.hbs          # Handlebars template (inline CSS)

tests/
├── integration.rs      # Integration tests with insta snapshots
├── snapshots/          # Insta snapshot files
└── fixtures/           # Sample JSON test data
    ├── basic_tests.json
    ├── mixed_output.json
    ├── error_cases.json
    └── ansi_colors.json
```

## Library API Design

```rust
/// Convert cargo test JSON output to HTML report
/// 
/// # Arguments
/// * `json_input` - Raw JSON string from cargo test output (may contain non-JSON lines)
/// * `config` - Configuration for HTML generation
/// 
/// # Returns
/// HTML string containing the test report, including any parsing errors
pub fn convert_to_html(json_input: &str, config: Config) -> String;

/// Trait for generating source code links
pub trait SourceLinker {
    fn link(&self, file: &str, line: u32) -> Option<String>;
}

/// Configuration for HTML generation
#[derive(bon::Builder)]
pub struct Config<L: SourceLinker = NoSourceLinker> {
    /// Source linker implementation
    #[builder(default)]
    pub source_linker: L,
}

/// Default no-op source linker
#[derive(Default)]
pub struct NoSourceLinker;

impl SourceLinker for NoSourceLinker {
    fn link(&self, _file: &str, _line: u32) -> Option<String> {
        None
    }
}

impl Default for Config<NoSourceLinker> {
    fn default() -> Self {
        Self {
            source_linker: NoSourceLinker,
        }
    }
}
```

## Error Handling Strategy
- **Never panic or abort**: Always produce HTML output
- **Graceful degradation**: Skip invalid JSON lines, continue processing
- **Error visibility**: Include parsing errors in HTML for user debugging
- **Raw line preservation**: Show non-JSON lines in a separate section

## Confirmed Requirements

✅ **Template Framework**: Handlebars  
✅ **ANSI Handling**: Defer until post-MVP, display raw output in `<pre>` tags  
✅ **CSS Framework**: No framework, self-contained HTML with inline CSS  
✅ **Output Format**: Single standalone HTML file  
✅ **Source Linking**: User-provided closure `Fn(&str, u32) -> String` for file+line  
✅ **Error Display**: Include full error context for debugging  
✅ **Config**: Use `#[bon]` builder, `Config` implements `Default`, no `Option<Config>`
