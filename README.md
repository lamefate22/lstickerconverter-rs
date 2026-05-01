# 🖼️ LStickerConverter-rs

A high-performance CLI tool written in **Rust** to convert and resize images into the Telegram sticker format (512px PNG). 🚀

## ✨ Features

- **⚡ Fast:** Utilizes parallel processing (via `rayon`) to convert multiple images simultaneously.
- **📏 Smart Resize:** Automatically scales images to 512px on the longest side while preserving aspect ratio.
- **🖼️ Formats:** Supports `JPG`, `JPEG`, `PNG`, `WEBP`, and `BMP`.
- **⌨️ Interactive Mode:** Simply run the app without arguments to be prompted for paths.

## 📥 Installation

### 📦 Download Binaries
You can download the pre-compiled executable for your platform from the [Releases](https://github.com/lamefate22/LStickerConverter-rs/releases) page.

### 🛠️ Build from Source
If you have [Rust and Cargo](https://rustup.rs/) installed:

```bash
git clone https://github.com/lamefate22/LStickerConverter-rs.git
cd lstickerconverter-rs
cargo build --release
```

The executable will be located at `target/release/lstickerconverter-rs`.

## 🚀 Usage

```bash
# 📁 Basic usage
lstickerconverter-rs --path "C:/Path/To/Images"

# 🛠️ Convert without resizing
lstickerconverter-rs --path "C:/Path/To/Images" --no-resize
```

## ⚙️ How it works

1. **🔍 Scan:** Looks for supported image formats in the provided directory.
2. **📁 Create:** Generates a `Converted` subdirectory.
3. **🎨 Process:** Resizes and converts images to PNG.
4. **💾 Save:** Stores results in the `Converted` folder.

## 📜 License

[MIT](LICENSE)