use clap::Parser;
use cargo_test_json_2_html::{convert_to_html, Config, SourceLinker};
use std::io::{self, Read};
use std::fs;

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

/// Example GitHub source linker
pub struct GitHubLinker {
    pub repo: String,
    pub branch: String,
}

impl SourceLinker for GitHubLinker {
    fn link(&self, file: &str, line: u32) -> Option<String> {
        Some(format!("https://github.com/{}/blob/{}/{}#L{}", self.repo, self.branch, file, line))
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
