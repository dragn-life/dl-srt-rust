# dl-srt-rust

Rust bindings for Haivision's SRT (Secure Reliable Transport) library, enabling low-latency video streaming in Rust applications.

## Status
🚧 **Code provided as is. Working only on what I need.** 🚧

## Features

- Safe Rust wrapper around SRT's native C/C++ API
- Core SRT functionality including:
    - Socket creation and management
    - Stream binding and listening
    - Data sending and receiving
    - Socket options handling
    - Error handling and status monitoring

## Prerequisites

- Rust 1.70 or higher
- SRT library (version 1.5.0 or higher)
    - Windows: Place `srt.dll` and `srt.lib` in the `lib` directory
    - Linux: Install via package manager or build from [source](https://github.com/Haivision/srt)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dl-srt-rust = { git = "https://github.com/dragn-life/dl-srt-rust" }
```

## Example Implementation

For a complete working example of these bindings in action, check out [dl-srt-server](https://github.com/dragn-life/dl-srt-server)

## API Examples

### Creating a Listener

```rust
let socket = SrtSocketConnection::new()?;
socket.bind(9000)?;
socket.listen(2)?;  // Backlog of 2
let accepted = socket.accept()?;
```

## Dependencies

This project uses:
- SRT (Secure Reliable Transport) library by Haivision (MPL 2.0)
    - Get it from: https://github.com/Haivision/srt
    - Windows: Requires srt.dll and srt.lib
    - Linux: Requires libsrt installed

## License

This project is licensed under the Mozilla Public License 2.0 (MPL 2.0).

## Contributing

Contributions are welcome! Please feel free to submit pull requests.

## Building From Source

1. Clone the repository
2. Ensure SRT library is available
3. Run `cargo build`

## Known Issues

- Windows: Ensure srt.dll is in your PATH or in the application directory
- Linux: Ensure libsrt is installed in your system

## Version History

- 0.1.0: Initial release
    - Basic SRT functionality
    - Windows support
    - Error handling

## Support

For issues and questions, please use the GitHub issue tracker.
