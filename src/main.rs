use cargo_test_json_2_html::{Config, SourceLinker, convert_to_html};
use clap::Parser;
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "cargo-test-json-2-html")]
#[command(about = "Convert cargo test JSON output to HTML report")]
struct Args {
    /// Input file (use - for stdin)
    #[arg(short, long, default_value = "-")]
    input: String,

    /// Output file (use - for stdout)
    #[arg(short, long, default_value = "-")]
    output: String,
}

/// Linker that generates links to GitHub
#[derive(Debug)]
pub struct GitHubLinker {
    pub repo: String,
    pub ref_: String, // branch, tag, or commit
}

impl GitHubLinker {
    pub fn new(repo: impl Into<String>) -> Self {
        Self {
            repo: repo.into(),
            ref_: "main".to_string(),
        }
    }

    pub fn with_ref(mut self, ref_: impl Into<String>) -> Self {
        self.ref_ = ref_.into();
        self
    }
}

impl SourceLinker for GitHubLinker {
    fn link(&self, file: &str, line: u32) -> Option<String> {
        Some(format!(
            "https://github.com/{}/blob/{}/{}#L{}",
            self.repo, self.ref_, file, line
        ))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read input
    let input = if args.input == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        fs::read_to_string(&args.input)?
    };

    // Configure (for now just use default, but could add CLI options for source linking)
    let config = Config::default();

    // Convert to HTML
    let html = convert_to_html(&input, config);

    // Write output
    if args.output == "-" {
        print!("{}", html);
    } else {
        fs::write(&args.output, html)?;
        eprintln!("Report written to {}", args.output);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pass() {
        println!("This test passes with stdout");
        eprintln!("This test has stderr output");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn test_fail() {
        println!("This test fails with stdout");
        eprintln!("This test has stderr output");
        assert_eq!(2 + 2, 5);
    }

    #[test]
    #[ignore]
    fn test_ignored() {
        assert_eq!(1, 1);
    }
}
