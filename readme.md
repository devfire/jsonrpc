# JSON-RPC Project

This project is a Rust-based implementation of a JSON-RPC server and client. JSON-RPC is a lightweight remote procedure call (RPC) protocol encoded in JSON. This implementation is designed to be efficient, extensible, and easy to integrate into your Rust applications.

## Features

- **JSON-RPC 2.0 Support**: Fully compliant with the JSON-RPC 2.0 specification.
- **Server and Client**: Includes both server and client implementations.
- **Asynchronous**: Built using Rust's async/await for high performance.
- **Customizable**: Easily extendable to support custom methods and middleware.
- **Error Handling**: Comprehensive error handling and reporting.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
jsonrpc = "0.1.0"
```

Then run:

```bash
cargo build
```

## Usage

### Server Example

```rust
use jsonrpc::server::{Server, Request, Response};

#[tokio::main]
async fn main() {
    let mut server = Server::new("127.0.0.1:8080");

    server.add_method("say_hello", |params| {
        Ok(Response::result(format!("Hello, {}!", params.get("name").unwrap_or("world"))))
    });

    server.run().await.unwrap();
}
```

### Client Example

```rust
use jsonrpc::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("http://127.0.0.1:8080");

    let response = client.call("say_hello", json!({"name": "Alice"})).await.unwrap();
    println!("Response: {}", response.result);
}
```

## Configuration

You can configure the server and client using environment variables or directly in the code. For example:

```rust
let server = Server::new("127.0.0.1:8080").with_max_connections(100);
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Submit a pull request with a detailed description.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [JSON-RPC Specification](https://www.jsonrpc.org/specification)
- Rust community for their support and libraries.

## Contact

For questions or support, please open an issue.