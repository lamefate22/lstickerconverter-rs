# 🖼️ LStickerConverter-rs

A high-performance, modern GUI tool written in **Rust** and **Tauri v2** to convert and resize images into the Telegram sticker format (512px PNG). 🚀

## ✨ Features

- **📏 Smart Resize:** Automatically scales images to 512px on the longest side while preserving aspect ratio.
- **⚡ Blazing Fast:** Utilizes parallel processing to convert dozens of images simultaneously.
- **🎨 Modern UI:** Minimalist, dark-themed interface with real-time progress updates.
- **🖼️ Wide Format Support:** Works with `JPG`, `JPEG`, `PNG`, `WEBP`, and `BMP`.
- **💻 Cross-Platform:** Built to run smoothly on Windows and Linux.

## 📥 Installation

### 📦 Download Binaries
You can download the pre-compiled executable for your platform from the [Releases](https://github.com/lamefate22/lstickerconverter-rs/releases) page.

### 🛠️ Build from Source
Ensure you have [Rust](https://rustup.rs/) and [Tauri dependencies](https://v2.tauri.app/start/prerequisites/) installed.

#### 1. Clone the repository
```bash
git clone https://github.com/lamefate22/lstickerconverter-rs.git
cd lstickerconverter-rs
```

#### 2. Linux Dependencies (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

#### 3. Compile
```bash
cd src-tauri
cargo build --release
```
The executable will be located in `src-tauri/target/release/`.

## 🚀 Usage

1. Launch the application.
2. Toggle the **Resize** option if needed.
3. Click **Choose Folder** and select the directory containing your images.
4. The tool will automatically create a `Converted` subfolder with your Telegram-ready PNGs.

## ⚙️ How it works

1. **Backend:** Scans the directory, manages the file system, and performs parallel image processing.
2. **Frontend:** A lightweight HTML/CSS/JS interface communicates with Rust via Tauri commands.
3. **Events:** Rust sends real-time progress events back to the UI to keep you informed.

## 📜 License

[MIT](LICENSE)
