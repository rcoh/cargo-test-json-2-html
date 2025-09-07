#!/bin/bash
set -e

# Generate example HTML report from complex test data
cargo run -- -i test-data/complex-example.json -o examples/test-report.html

echo "Example report generated at examples/test-report.html"
