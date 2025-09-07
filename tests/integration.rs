use cargo_test_json_2_html::{Config, SourceLinker, convert_to_html};
use std::fs;
use proptest::prelude::*;

#[derive(Debug)]
struct TestLinker;

impl SourceLinker for TestLinker {
    fn link(&self, file: &str, line: u32) -> Option<String> {
        Some(format!(
            "https://github.com/test/repo/blob/main/{}#L{}",
            file, line
        ))
    }
}

proptest! {
    #[test]
    fn convert_to_html_never_panics(input in ".*") {
        let config = Config::default();
        let _ = convert_to_html(&input, config);
    }
}

#[test]
fn test_xss_protection() {
    let input = fs::read_to_string("test-data/xss-protection.json").unwrap();
    let config = Config::default();
    let html = convert_to_html(&input, config);

    // Should not contain unescaped script tags
    assert!(!html.contains("<script>alert('xss')</script>"));
    // Should contain escaped version
    assert!(html.contains("&lt;script&gt;"));

    insta::assert_snapshot!(html);
}

#[test]
fn test_source_linking() {
    let input = fs::read_to_string("test-data/source-linking.json").unwrap();
    let config = Config::builder().source_linker(TestLinker).build();
    let html = convert_to_html(&input, config);

    insta::assert_snapshot!(html);
}

#[test]
fn test_real_cargo_output() {
    let input = fs::read_to_string("test-data/basic.json").unwrap();
    let config = Config::default();
    let html = convert_to_html(&input, config);

    insta::assert_snapshot!(html);
}

#[test]
fn test_mixed_output_with_compilation() {
    let input = fs::read_to_string("test-data/mixed-compilation.json").unwrap();
    let config = Config::default();
    let html = convert_to_html(&input, config);

    insta::assert_snapshot!(html);
}

#[test]
fn test_error_handling() {
    let input = fs::read_to_string("test-data/error-handling.json").unwrap();
    let config = Config::default();
    let html = convert_to_html(&input, config);

    insta::assert_snapshot!(html);
}
