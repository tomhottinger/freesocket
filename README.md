# freesocket

**freesocket** is a lightweight command-line utility that finds and prints a free TCP port available for listening on the local machine. Intended for Scripting. It's written in RUST so it can be considered safe. 

Written by Tom Hottinger (artScape cybernetics).

---

## Features

- Instantly finds a free TCP port and prints it to stdout
- Optional port range restriction via `--min` and `--max`
- Clean output — only the port number goes to stdout, errors go to stderr
- Works on Windows and Linux
- No external dependencies

## Installation

### Pre-built binary

Download the latest release from the [Releases](https://github.com/tomhottinger/freesocket/releases) page.

### Build from source

Requires [Rust](https://www.rust-lang.org/tools/install) (stable).

```bash
git clone https://github.com/tomhottinger/freesocket.git
cd freesocket
cargo build --release
```

The binary will be at `target/release/freesocket` (Linux) or `target\release\freesocket.exe` (Windows).

## Usage

```
freesocket [OPTIONS]

OPTIONS:
  --min <PORT>        Minimum port number (default: 1024)
  --max <PORT>        Maximum port number (default: 65535)
  --help, -h, /?, -?  Show help message
```

### Examples

```bash
# Find any free user port
freesocket

# Find a free port in a specific range
freesocket --min 8000 --max 9000

# Find a free port starting from 3000
freesocket --min 3000
```

### Using the output in scripts

Because the port number is the only thing written to stdout, you can capture it directly:

```bash
# Bash
PORT=$(freesocket --min 8000 --max 9000)
echo "Starting server on port $PORT"
```

```powershell
# PowerShell
$PORT = .\freesocket.exe --min 8000 --max 9000
Write-Host "Starting server on port $PORT"
```

## Exit codes

| Code | Meaning |
|------|---------|
| `0`  | A free port was found and printed |
| `1`  | No free port found in the given range, or invalid arguments |

## Disclaimer

This software is provided "as is", without warranty of any kind. The author and copyright holders are not responsible for any damage, data loss, or other issues arising from the use of this software. Use at your own risk.

The tool binds briefly to a port to test availability and releases it immediately. In rare cases, another process may claim the port between the check and your actual use of it.

## License

MIT License

Copyright (c) 2026 Tom Hottinger (artScape cybernetics)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
