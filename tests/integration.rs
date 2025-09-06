use cargo_test_json_2_html::{convert_to_html, Config};

#[test]
fn test_real_cargo_output() {
    let input = r#"{ "type": "suite", "event": "started", "test_count": 3 }
{ "type": "test", "event": "started", "name": "tests::test_fail" }
{ "type": "test", "event": "started", "name": "tests::test_ignored" }
{ "type": "test", "event": "started", "name": "tests::test_pass" }
{ "type": "test", "name": "tests::test_ignored", "event": "ignored" }
{ "type": "test", "name": "tests::test_pass", "event": "ok", "stdout": "This test passes with stdout\nThis test has stderr output\n" }
{ "type": "test", "name": "tests::test_fail", "event": "failed", "stdout": "This test fails with stdout\nThis test has stderr output\n\nthread 'tests::test_fail' panicked at src/main.rs:18:9:\nassertion `left == right` failed\n  left: 4\n right: 5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n" }
{ "type": "suite", "event": "failed", "passed": 1, "failed": 1, "ignored": 1, "measured": 0, "filtered_out": 0, "exec_time": 0.000290792 }"#;

    let config = Config::default();
    let html = convert_to_html(input, config);
    
    insta::assert_snapshot!(html);
}

#[test]
fn test_mixed_output_with_compilation() {
    let input = r#"   Compiling cargo-test-json-2-html v0.1.0 (/Users/rcoh/cargo-test-json-2-html)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running unittests src/main.rs (target/debug/deps/cargo_test_json_2_html-f06c6ae57aba77b0)

{ "type": "suite", "event": "started", "test_count": 1 }
{ "type": "test", "event": "started", "name": "tests::simple_test" }
{ "type": "test", "name": "tests::simple_test", "event": "ok", "stdout": "test output\n" }
{ "type": "suite", "event": "ok", "passed": 1, "failed": 0, "ignored": 0, "measured": 0, "filtered_out": 0, "exec_time": 0.001 }"#;

    let config = Config::default();
    let html = convert_to_html(input, config);
    
    insta::assert_snapshot!(html);
}

#[test]
fn test_error_handling() {
    let input = r#"{ "type": "test", "invalid": "json" }
{ "type": "test", "name": "valid_test", "event": "ok" }
{ not json at all
Some compilation output"#;

    let config = Config::default();
    let html = convert_to_html(input, config);
    
    insta::assert_snapshot!(html);
}
