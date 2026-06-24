use std::net::{IpAddr, TcpListener};

const DEFAULT_MIN: u16 = 1024;
const DEFAULT_MAX: u16 = 65535;
const DEFAULT_IP: &str = "0.0.0.0";

fn print_help(program: &str) {
    eprintln!("freesocket, written by Tom Hottinger (artScape cybernetics). All rights reserved.");
    eprintln!("For details check GitHub: https://github.com/tomhottinger/freesocket");
    eprintln!();
    eprintln!("Usage: {} [OPTIONS]", program);
    eprintln!();
    eprintln!("Finds a free TCP port available for listening and prints it to stdout.");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("  --ip  <IP>          IP address to bind to (default: {})", DEFAULT_IP);
    eprintln!("                      Use 127.0.0.1 for loopback, 0.0.0.0 for all IPv4 interfaces.");
    eprintln!("                      IPv6 is supported, e.g. ::1 (loopback) or :: (all interfaces).");
    eprintln!("  --min <PORT>        Minimum port number (default: {})", DEFAULT_MIN);
    eprintln!("  --max <PORT>        Maximum port number (default: {})", DEFAULT_MAX);
    eprintln!("  --help, -h, /?, -?  Show this help message");
    eprintln!();
    eprintln!("EXAMPLES:");
    eprintln!("  {}                            Find any free user port on all interfaces", program);
    eprintln!("  {} --ip 127.0.0.1             Only check loopback (IPv4)", program);
    eprintln!("  {} --ip ::1                   Only check loopback (IPv6)", program);
    eprintln!("  {} --ip 0.0.0.0 --min 8000 --max 9000", program);
}

fn find_free_port(ip: IpAddr, min: u16, max: u16) -> Option<u16> {
    (min..=max).find(|&port| TcpListener::bind((ip, port)).is_ok())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args.first().map(|s| s.as_str()).unwrap_or("freesocket");

    let mut min = DEFAULT_MIN;
    let mut max = DEFAULT_MAX;
    let mut ip: IpAddr = DEFAULT_IP.parse().unwrap();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" | "/?" | "-?" => {
                print_help(program);
                std::process::exit(0);
            }
            "--ip" => {
                i += 1;
                match args.get(i).and_then(|v| v.parse::<IpAddr>().ok()) {
                    Some(v) => ip = v,
                    None => {
                        eprintln!("Error: --ip requires a valid IP address (e.g. 127.0.0.1 or ::1)");
                        std::process::exit(1);
                    }
                }
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

    match find_free_port(ip, min, max) {
        Some(port) => println!("{}", port),
        None => {
            eprintln!("Error: no free port found in range {}-{} on {}", min, max, ip);
            std::process::exit(1);
        }
    }
}
