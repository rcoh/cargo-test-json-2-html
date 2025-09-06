use serde::{Deserialize, Serialize};
use handlebars::Handlebars;

/// Trait for generating source code links
pub trait SourceLinker {
    fn link(&self, file: &str, line: u32) -> Option<String>;
}

/// Configuration for HTML generation
#[derive(bon::Builder)]
pub struct Config {
    /// Source linker implementation
    #[builder(default = Box::new(NoSourceLinker))]
    pub source_linker: Box<dyn SourceLinker>,
}

/// Default no-op source linker
#[derive(Default)]
pub struct NoSourceLinker;

impl SourceLinker for NoSourceLinker {
    fn link(&self, _file: &str, _line: u32) -> Option<String> {
        None
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            source_linker: Box::new(NoSourceLinker),
        }
    }
}

/// Cargo test JSON event types
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum TestEvent {
    #[serde(rename = "suite")]
    Suite {
        event: String,
        test_count: Option<u32>,
        passed: Option<u32>,
        failed: Option<u32>,
        ignored: Option<u32>,
        measured: Option<u32>,
        filtered_out: Option<u32>,
        exec_time: Option<f64>,
    },
    #[serde(rename = "test")]
    Test {
        event: String,
        name: String,
        stdout: Option<String>,
        exec_time: Option<f64>,
    },
}

/// Parsed test results
#[derive(Debug, Default)]
pub struct TestResults {
    pub passed: Vec<TestEvent>,
    pub failed: Vec<TestEvent>,
    pub ignored: Vec<TestEvent>,
    pub suite_info: Option<TestEvent>,
    pub errors: Vec<String>,
    pub raw_lines: Vec<String>,
}

/// Template data for rendering
#[derive(Serialize)]
struct TemplateData {
    passed: Vec<TestEvent>,
    failed: Vec<TestEvent>,
    ignored: Vec<TestEvent>,
    suite_info: Option<TestEvent>,
    errors: Vec<String>,
    raw_lines: Vec<String>,
    passed_count: usize,
    failed_count: usize,
    ignored_count: usize,
}

/// Convert cargo test JSON output to HTML report
/// 
/// # Arguments
/// * `json_input` - Raw JSON string from cargo test output (may contain non-JSON lines)
/// * `config` - Configuration for HTML generation
/// 
/// # Returns
/// HTML string containing the test report, including any parsing errors
pub fn convert_to_html(json_input: &str, config: Config) -> String {
    let results = parse_test_output(json_input);
    render_html(&results, &config)
}

fn parse_test_output(input: &str) -> TestResults {
    let mut results = TestResults::default();
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        match serde_json::from_str::<TestEvent>(line) {
            Ok(event) => {
                match &event {
                    TestEvent::Suite { .. } => {
                        results.suite_info = Some(event);
                    }
                    TestEvent::Test { event: status, .. } => {
                        match status.as_str() {
                            "ok" => results.passed.push(event),
                            "failed" => results.failed.push(event),
                            "ignored" => results.ignored.push(event),
                            _ => results.raw_lines.push(line.to_string()),
                        }
                    }
                }
            }
            Err(e) => {
                // Not JSON or invalid JSON - could be compilation output
                if line.starts_with('{') {
                    results.errors.push(format!("Failed to parse JSON: {} - Line: {}", e, line));
                } else {
                    results.raw_lines.push(line.to_string());
                }
            }
        }
    }
    
    results
}

fn render_html(results: &TestResults, _config: &Config) -> String {
    let template_str = include_str!("../templates/report.hbs");
    
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("report", template_str)
        .expect("Failed to register template");
    
    let data = TemplateData {
        passed_count: results.passed.len(),
        failed_count: results.failed.len(),
        ignored_count: results.ignored.len(),
        passed: results.passed.clone(),
        failed: results.failed.clone(),
        ignored: results.ignored.clone(),
        suite_info: results.suite_info.clone(),
        errors: results.errors.clone(),
        raw_lines: results.raw_lines.clone(),
    };
    
    handlebars.render("report", &data)
        .unwrap_or_else(|e| format!("<html><body><h1>Template Error</h1><p>{}</p></body></html>", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let input = r#"{ "type": "suite", "event": "started", "test_count": 3 }
{ "type": "test", "event": "started", "name": "tests::test_pass" }
{ "type": "test", "name": "tests::test_pass", "event": "ok", "stdout": "This test passes\n" }
{ "type": "test", "event": "started", "name": "tests::test_fail" }
{ "type": "test", "name": "tests::test_fail", "event": "failed", "stdout": "This test fails\n" }
{ "type": "suite", "event": "failed", "passed": 1, "failed": 1, "ignored": 0 }"#;

        let results = parse_test_output(input);
        assert_eq!(results.passed.len(), 1);
        assert_eq!(results.failed.len(), 1);
        assert_eq!(results.ignored.len(), 0);
        assert!(results.suite_info.is_some());
    }

    #[test]
    fn test_mixed_content() {
        let input = r#"   Compiling test-project v0.1.0
{ "type": "test", "name": "tests::test_pass", "event": "ok" }
Some non-JSON output
{ "type": "suite", "event": "ok", "passed": 1, "failed": 0 }"#;

        let results = parse_test_output(input);
        assert_eq!(results.passed.len(), 1);
        assert_eq!(results.raw_lines.len(), 2); // Compilation line + non-JSON line
    }

    #[test]
    fn test_convert_to_html_basic() {
        let input = r#"{ "type": "test", "name": "test", "event": "ok" }"#;
        let config = Config::default();
        let html = convert_to_html(input, config);
        assert!(html.contains("<div class=\"stat-number\">1</div>"));
        assert!(html.contains("<div class=\"stat-label\">Passed</div>"));
        assert!(html.contains("<div class=\"stat-number\">0</div>"));
        assert!(html.contains("<div class=\"stat-label\">Failed</div>"));
    }
}
