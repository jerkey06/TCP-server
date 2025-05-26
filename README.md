# Server TCP

A simple TCP server implementation in Rust that listens for incoming connections and displays received messages.

## Features

- Listens on all network interfaces (0.0.0.0) on port 8888
- Accepts multiple client connections
- Displays incoming messages in UTF-8 format
- Handles client disconnections gracefully
- Built with pure Rust standard library (no external dependencies)

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/server-tcp.git
   cd server-tcp
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Running the server

```bash
cargo run
```

Or, after building:

```bash
./target/release/server-tcp
```

The server will start listening on port 8888 on all network interfaces.

### Connecting to the server

You can connect to the server using tools like telnet, netcat, or any TCP client:

```bash
# Using netcat
nc localhost 8888

# Using telnet
telnet localhost 8888
```

Type messages and press Enter to send them to the server. The server will display these messages in its console output.

## How It Works

The server creates a TCP listener bound to 0.0.0.0:8888. For each incoming connection, it spawns a handler that reads data from the client in chunks of 512 bytes and prints the received data to the console.

The server handles client disconnections and connection errors appropriately, printing status messages to the console.

### Command Protocol

The server implements a simple command protocol that processes incoming messages and returns appropriate responses:

| Command | Description | Response |
|---------|-------------|----------|
| `000` | Welcome command | `Welcome user` |
| `001` | Connection confirmation | `Operation: connected!` |
| `002 PUSH` | Push operation | `Operation: PUSH \| Success` |
| `002 PULL` | Pull operation | `Operation: PULL \| Success` |

Any unrecognized command will return `Operation: Unknown`.

#### Example Usage:

```
# Send welcome command
000

# Confirm connection
001

# Execute push operation
002 PUSH

# Execute pull operation
002 PULL
```

The server parses each command, extracts any arguments, and processes them according to the protocol rules. Command responses are sent back to the client.

## Project Structure

- `src/main.rs` - Contains the server implementation
- `Cargo.toml` - Project configuration and dependencies

## License

[MIT License](LICENSE) (or specify your preferred license)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
