# WebP Validator

Rust实现的WebP图片格式校验库，提供C FFI接口供Go等其他语言调用。

## 项目结构

```
webp_validator/
├── src/                    # Rust源码
│   ├── lib.rs              # Rust库（FFI接口）
│   └── main.rs             # Rust示例
├── include/                # C头文件
│   └── webp_validator.h
├── lib/                    # 动态库目录
│   ├── webp_validator.dll      # Windows
│   └── libwebp_validator.so    # Linux
├── go_pkg/                 # Go示例和测试
│   ├── main.go
│   ├── validator_windows.go
│   ├── validator_linux.go
│   └── validator_test.go
├── images/                 # 测试图片
└── Cargo.toml
```

## 功能特性

- ✅ 校验WebP格式合法性
- ✅ 检测动态/静态WebP
- ✅ 提取元数据（宽高、透明通道、帧数）
- ✅ 识别伪造WebP文件
- ✅ C FFI接口，支持多语言调用

---

## 快速开始

### 编译动态库

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

### Rust使用

```bash
# 运行测试
cargo test

# 运行示例
cargo run
```

### Go使用

**运行示例:**
```bash
cd go_pkg

# Windows
$env:PATH = "$(Resolve-Path ..\lib);$env:PATH"
go run .

# Linux（已配置rpath，无需设置环境变量）
go run .
```

**运行测试:**
```bash
cd go_pkg

# Windows
$env:PATH = "$(Resolve-Path ..\lib);$env:PATH"
go test -v

# Linux
go test -v

# 标准库对比测试（证明stdlib无法处理动态WebP）
go test -v -run TestCompareWithStdLib
```

---

## API示例

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

## 部署

### 开发环境

**Windows:**
- 将 `lib` 目录添加到PATH环境变量
- 或复制DLL到可执行文件同目录

**Linux:**
- Go程序已配置rpath，自动从 `../lib` 加载
- 或设置 `LD_LIBRARY_PATH=../lib`

### 生产环境

**Linux系统级安装:**
```bash
sudo cp lib/libwebp_validator.so /usr/local/lib/
sudo ldconfig
```

---

## 常见问题

**Q: Windows运行时找不到DLL？**

A: 添加lib目录到PATH：
```powershell
$env:PATH = "$(Resolve-Path lib);$env:PATH"
```

**Q: Linux加载.so失败？**

A: Go程序已配置rpath，直接运行即可。或手动设置：
```bash
export LD_LIBRARY_PATH=lib:$LD_LIBRARY_PATH
```

**Q: Go找不到头文件？**

A: 确保 `include/webp_validator.h` 存在，Go代码引用路径为 `../include/webp_validator.h`

**Q: 如何验证动态库？**

```bash
# Linux
file lib/libwebp_validator.so
nm -D lib/libwebp_validator.so | grep validate_webp_ffi

# Windows
dir lib\webp_validator.dll
```

---

## 技术亮点

### Go标准库对比

项目包含测试证明Go标准库 `golang.org/x/image/webp` 无法处理动态WebP：

```bash
go test -v -run TestCompareWithStdLib
```

- ✅ Rust库：完整支持动态WebP，提取帧数等元数据
- ❌ Go标准库：无法处理动态WebP文件（会报错）

### 平台适配

- **Windows**: 使用 `validator_windows.go`
- **Linux**: 使用 `validator_linux.go`，配置rpath自动加载库

---

## 许可证

MIT License
