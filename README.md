# 🚀 Axiom Organon
A lightning-fast, zero-bloat file infrastructure organizer written in pure Rust. Stop wasting memory on heavy, slow Electron-based desktop tools just to keep your workspace clean. **Axiom Organon** structures mixed environments, source files, and binary assets instantly from the bare metal.
## 🔥 Features
- **Zero Dependencies:** Written using only Rust's native file system APIs (`std::fs`).
- **Low Latency:** Processes thousands of unsorted directory entries in milliseconds.
- **Smart Mapping:** Automatically classifies code repositories, heavy media assets, archives, installers, and documents into strict directories based on modern extension signatures.
- **Ultra-Lightweight:** Compiled directly to silicon with zero memory leak overhead.
## 📁 How It Organizes Your Workspace
The engine scans the target directory and automatically classifies files into the following matrix:

| Category | Extensions | Target Directory |
| :--- | :--- | :--- |
| **Source Code** | `.rs`, `.py`, `.js`, `.cpp`, `.json`, `.html`, `.css` | `Projects_Source/` |
| **Media Assets** | `.jpg`, `.jpeg`, `.png`, `.gif`, `.mp4`, `.mov`, `.mp3` | `Media_Files/` |
| **Documents** | `.pdf`, `.docx`, `.txt`, `.xlsx`, `.pptx` | `Documents/` |
| **Archives** | `.zip`, `.tar`, `.gz`, `.rar`, `.7z` | `Archives/` |
| **Installers** | `.exe`, `.msi`, `.deb`, `.dmg` | `Installers/` |

## 🛠️ Quick Start

### 1. Build from Source
Ensure you have the Rust toolchain installed, then clone and build:

```bash
git clone [https://github.com/echeparesmanuel36-design/axiom-organon.git](https://github.com/echeparesmanuel36-design/axiom-organon.git)
```
```bash
cd axiom-organon
```
```bash
cargo build --release
```

### 2. Execution

​Move the compiled binary to any cluttered directory (like your Downloads folder) and run it:
```bash
./axiom-organon
```
## ⚡ The Philosophy

​Mainstream software is broken. We don't need cloud infrastructure, complex telemetry, or heavy UI frameworks just to sort files on a local disk. Axiom Organon is built on the core Axiom Systems principles: raw execution, memory safety, and absolute respect for CPU cycles.

​Built for developers who care about local hardware execution.
