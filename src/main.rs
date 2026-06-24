use std::net::TcpListener;

const DEFAULT_MIN: u16 = 1024;
const DEFAULT_MAX: u16 = 65535;

fn print_help(program: &str) {
    eprintln!("freesocket, written by Tom Hottinger (artScape cybernetics). All rights reserved.");
    eprintln!("For details check GitHub: https://github.com/tomhottinger/freesocket");
    eprintln!();
    eprintln!("Usage: {} [OPTIONS]", program);
    eprintln!();
    eprintln!("Finds a free TCP port available for listening and prints it to stdout.");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("  --min <PORT>        Minimum port number (default: {})", DEFAULT_MIN);
    eprintln!("  --max <PORT>        Maximum port number (default: {})", DEFAULT_MAX);
    eprintln!("  --help, -h, /?, -?  Show this help message");
    eprintln!();
    eprintln!("EXAMPLES:");
    eprintln!("  {}                       Find any free user port", program);
    eprintln!("  {} --min 8000 --max 9000  Search only in 8000-9000", program);
    eprintln!("  {} --min 3000             Search from port 3000 upward", program);
}

fn find_free_port(min: u16, max: u16) -> Option<u16> {
    (min..=max).find(|&port| TcpListener::bind(("127.0.0.1", port)).is_ok())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args.first().map(|s| s.as_str()).unwrap_or("freesocket");

    let mut min = DEFAULT_MIN;
    let mut max = DEFAULT_MAX;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" | "/?" | "-?" => {
                print_help(program);
                std::process::exit(0);
            }
            "--min" => {
                i += 1;
                match args.get(i).and_then(|v| v.parse::<u16>().ok()) {
                    Some(v) => min = v,
                    None => {
                        eprintln!("Error: --min requires a valid port number (1-65535)");
                        std::process::exit(1);
                    }
                }
            }
            "--max" => {
                i += 1;
                match args.get(i).and_then(|v| v.parse::<u16>().ok()) {
                    Some(v) => max = v,
                    None => {
                        eprintln!("Error: --max requires a valid port number (1-65535)");
                        std::process::exit(1);
                    }
                }
            }
            other => {
                eprintln!("Error: unknown argument '{}'", other);
                eprintln!("Use --help for usage information.");
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if min > max {
        eprintln!("Error: --min ({}) must not be greater than --max ({})", min, max);
        std::process::exit(1);
    }

    match find_free_port(min, max) {
        Some(port) => println!("{}", port),
        None => {
            eprintln!("Error: no free port found in range {}-{}", min, max);
            std::process::exit(1);
        }
    }
}
