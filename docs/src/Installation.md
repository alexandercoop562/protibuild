# Installation

This guide covers how to install and run Protibuild on your system.

## Prerequisites

Before installing Protibuild, ensure your system meets the requirements:

### System Requirements

- **Operating System**: Windows, macOS, or Linux
- **Rust**: Latest stable Rust (see [rustup](https://rustup.rs/))
- **Graphics**: WebGL 2.0 capable GPU

### Bevy Dependencies

Protibuild uses the Bevy game engine, which requires certain system dependencies. Follow the [Bevy setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/) for your specific OS.

Common dependencies include:
- On Linux: `libudev`, `libasound2`, `libxkbcommon`
- On macOS: Xcode command line tools
- On Windows: Visual Studio Build Tools

## Installation Methods

### From Source (Recommended)

1. **Clone the repository**

   ```bash
   git clone https://github.com/alexandercoop562/protibuild.git
   cd protibuild
   ```

2. **Build and run**

   ```bash
   make run
   ```

   Or directly:

   ```bash
   cargo run -p protibuild
   ```

### Using Make Commands

The project includes a Makefile with common commands:

| Command | Description |
|---------|-------------|
| `make run` | Build and run the application |
| `make qa` | Run all quality checks (fmt, clippy, audit, deny) |

### Code Quality Commands

If you want to check code quality separately:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Security audit
cargo audit

# License check
cargo deny check
```

## Running the Application

### Development Build

For development with faster compile times, use dynamic linking:

```bash
cargo run -p protibuild
```

The application will launch with a window showing the 3D viewport.

### First Launch

On first launch:

1. A window opens showing the "Dev Cube" project
2. Click the **File** tab to access project templates
3. Press **C** to capture your cursor and enable camera controls
4. Use **WASD** + **Space/Shift** to move around

## Troubleshooting

### Compilation Errors

If you encounter compilation errors:

1. Ensure Rust is up to date: `rustup update`
2. Clear build cache: `cargo clean`
3. Rebuild: `cargo build -p protibuild`

### Missing Dependencies

If the application fails to start with missing library errors:

- **Linux**: Install system packages for Bevy (see Bevy setup guide)
- **macOS**: Run `xcode-select --install`
- **Windows**: Ensure Visual Studio Build Tools are installed

### Performance Issues

If the application runs slowly:

- Ensure GPU drivers are up to date
- Try running in release mode: `cargo run -p protibuild --release`
