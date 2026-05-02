# LStickerConverter TODO

## 🏗 1. Architecture & Reliability
- [ ] **Advanced Error Handling**: 
    - Implement `serde::Serialize` for `ApplicationErrors` to pass structured error data to UI.
    - Add localized error messages.
- [ ] **Logging & Debugging**:
    - Integrate `tauri-plugin-log` for file-based logging.
    - Log every failed file conversion with reasons.
- [ ] **State Persistence**:
    - Integrate `tauri-plugin-store` to remember "Resize" toggle and "Output Format" settings across restarts.
- [ ] **Validation & Testing**:
    - Add unit tests for `filesystem.rs` and `converter.rs`.
    - Add integration tests for the conversion command.

## 🚀 2. New Features
- [ ] **WebP Support**:
    - Add option to export as `.webp` (smaller size, native Telegram support).
    - Use `fast_image_resize` for better Lanczos3 scaling.
- [ ] **Batch Rename**:
    - Add setting for custom prefix/suffix (e.g., `sticker_01.png`).

## 🎨 UI/UX Enhancements
- [ ] **Preview**: Show a small thumbnail of the first image found in the selected folder.
- [ ] **Drag-and-Drop**: Support dropping a folder directly into the app window.
- [ ] **Theme Support**: Sync with system Dark/Light mode.
