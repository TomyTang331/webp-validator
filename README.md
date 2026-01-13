# WebP Validator

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Go](https://img.shields.io/badge/Go-1.18%2B-00ADD8.svg)](https://golang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey.svg)](https://github.com/TomyZhang331/WebpValidator)

A high-performance WebP image format validator built with Rust, featuring C FFI interface for seamless cross-language integration. This library provides comprehensive WebP validation capabilities, including support for animated WebP files that Go's standard library cannot handle.

## Key Features

- ✅ **Full WebP Support**: Validate both static and animated WebP formats
- ✅ **Metadata Extraction**: Get image dimensions, alpha channel info, and frame counts
- ✅ **Fake Detection**: Identify incorrectly renamed non-WebP files
- ✅ **Cross-Language**: C FFI interface for use in Go, Python, Node.js, and more
- ✅ **High Performance**: Built with Rust for optimal speed and safety
- ✅ **Zero Dependencies**: Minimal runtime dependencies for easy deployment

## Project Structure

```
webp_validator/
├── src/                    # Rust source code
│   ├── lib.rs              # Rust library with FFI interface
│   └── main.rs             # Rust example
├── include/                # C header files
│   └── webp_validator.h
├── lib/                    # Dynamic library directory
│   ├── webp_validator.dll      # Windows
│   └── libwebp_validator.so    # Linux
├── go_pkg/                 # Go examples and tests
│   ├── main.go
│   ├── validator_windows.go
│   ├── validator_linux.go
│   └── validator_test.go
├── images/                 # Test images
└── Cargo.toml
```

---

## Installation & Setup

### Build Dynamic Library

**Windows:**
```powershell
cargo build --release --lib
mkdir lib
copy target\release\webp_validator.dll lib\
```

**Linux:**
```bash
cargo build --release --lib
mkdir -p lib
cp target/release/libwebp_validator.so lib/
```

### Rust Usage

```bash
# Run tests
cargo test

# Run example
cargo run
```

### Go Usage

**Run Example:**
```bash
cd go_pkg

# Windows
$env:PATH = "$(Resolve-Path ..\lib);$env:PATH"
go run .

# Linux (requires LD_LIBRARY_PATH)
export LD_LIBRARY_PATH=../lib:$LD_LIBRARY_PATH
go run .
```

**Run Tests:**
```bash
cd go_pkg

# Windows
$env:PATH = "$(Resolve-Path ..\lib);$env:PATH"
go test -v

# Linux
export LD_LIBRARY_PATH=../lib:$LD_LIBRARY_PATH
go test -v

# Compare with Go stdlib (proves stdlib cannot handle animated WebP)
LD_LIBRARY_PATH=../lib go test -v -run TestCompareWithStdLib
```

---

## API Examples

### Rust API

```rust
use webp_validator::validate_webp;

match validate_webp("test.webp") {
    Ok(info) => {
        println!("{}x{}", info.width, info.height);
        println!("animated: {}", info.is_animated);
        if info.is_animated {
            println!("frames: {}", info.num_frames);
        }
    }
    Err(e) => println!("error: {}", e),
}
```

### Go API

```go
info := ValidateWebp("test.webp")
if info.IsValid {
    fmt.Printf("%dx%d\n", info.Width, info.Height)
    if info.IsAnimated {
        fmt.Printf("frames: %d\n", info.NumFrames)
    }
} else {
    fmt.Println(info.Error)
}
```

---

## Deployment

### Development Environment

**Windows:**
- Add `lib` directory to PATH environment variable
- Or copy DLL to executable directory

**Linux:**
- Set `LD_LIBRARY_PATH` to include `lib` directory:
  ```bash
  export LD_LIBRARY_PATH=/path/to/project/lib:$LD_LIBRARY_PATH
  ```

### Production Environment

**Linux System-wide Installation:**
```bash
sudo cp lib/libwebp_validator.so /usr/local/lib/
sudo ldconfig
```

**Then run Go programs without LD_LIBRARY_PATH:**
```bash
go run .
go test -v
```

---

## FAQ

**Q: Windows can't find DLL at runtime?**

A: Add lib directory to PATH:
```powershell
$env:PATH = "$(Resolve-Path lib);$env:PATH"
```

**Q: Linux fails to load .so file?**

A: Set LD_LIBRARY_PATH environment variable:
```bash
export LD_LIBRARY_PATH=../lib:$LD_LIBRARY_PATH
# Or use inline:
LD_LIBRARY_PATH=../lib go test -v
```

For system-wide installation, copy to `/usr/local/lib/` and run `ldconfig`.

**Q: Go can't find header file?**

A: Ensure `include/webp_validator.h` exists. Go code references it as `../include/webp_validator.h`.

**Q: How to verify the dynamic library?**

```bash
# Linux
file lib/libwebp_validator.so
nm -D lib/libwebp_validator.so | grep validate_webp_ffi

# Windows
dir lib\webp_validator.dll
```

---

## Technical Highlights

### Go Standard Library Comparison

This project includes tests that prove Go's standard library `golang.org/x/image/webp` cannot handle animated WebP:

```bash
LD_LIBRARY_PATH=../lib go test -v -run TestCompareWithStdLib
```

**Test Results:**
- ✅ **Rust library**: Full animated WebP support, extracts frame count and metadata
- ❌ **Go stdlib**: Cannot decode animated WebP files (returns error)

**Example Output:**
```
=== RUN   TestCompareWithStdLib
    validator_test.go:87: rust library result: valid=true, animated=true, frames=46
    validator_test.go:94: golang stdlib result: error (expected) - invalid image file: webp: invalid format
    validator_test.go:95: this proves golang stdlib cannot handle animated webp files
--- PASS: TestCompareWithStdLib (0.00s)
```

### Platform Adaptation

- **Windows**: Uses `validator_windows.go` with PATH-based library loading
- **Linux**: Uses `validator_linux.go` with rpath configuration (requires LD_LIBRARY_PATH for `go test`)

---

## License

MIT License
