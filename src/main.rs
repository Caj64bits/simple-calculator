use colored::{self, Colorize};
use std::io::Write;

fn print_help() {
    println!(
        "  • Basic Math:     {} or {}",
        "5 + 3 * 2".cyan(),
        "(10 - 4) / 2".cyan()
    );
    println!("  • Square Roots:   {}", "sqrt(16)".cyan());
    println!("  • Powers:         {}", "2^3".cyan());
    println!("  • Constants:      {} or {}", "pi".cyan(), "e".cyan());
    println!(
        "  • Exit:           Type {}, {}, or {}\n",
        "q".red(),
        "quit".red(),
        "exit".red()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                return; // Exit the program after showing help
            }
            "--version" | "-v" => {
                println!("{} v0.1.0", "Simple Calculator".blue());
                return; // Exit the program after showing version
            }
            other => {
                println!(
                    "{} Unknown flag '{}'. Use --help for usage.",
                    "Error:".red(),
                    other
                );
                return;
            }
        }
    }

    println!("{}", "Simple Calculator".blue());

    loop {
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        let mut raw_input = String::new();

        std::io::stdin()
            .read_line(&mut raw_input)
            .expect("Failed to read input");

        let input = raw_input.trim();

        if input.eq_ignore_ascii_case("q")
            || input.eq_ignore_ascii_case("quit")
            || input.eq_ignore_ascii_case("exit")
        {
            break;
        }

        // 2. Handle built-in help doc
        if input.eq_ignore_ascii_case("help") {
            print_help();
            continue;
        }

        if input.is_empty() {
            continue;
        }
        match meval::eval_str(&raw_input) {
            Ok(result) => println!("{}", result.to_string().green()),
            Err(err) => println!("{}", format!("{}", err).red()),
        }
    }
}
