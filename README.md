<p align="center">
  <img src="assets/banner.png" alt="ATPspy Banner" width="700"/>
</p>

<h3 align="center">🔍 Your APT updates, decoded.</h3>

<p align="center">
  A terminal UI (TUI) app that gives you full visibility into your <code>apt</code> package updates — what you're updating, how critical it is, and what changes it brings.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust" alt="Rust"/>
  <img src="https://img.shields.io/badge/TUI-ratatui-cyan" alt="Ratatui"/>
  <img src="https://img.shields.io/badge/Platform-Linux-green?logo=linux" alt="Linux"/>
  <img src="https://img.shields.io/github/license/Parashar-dev/ATPspy" alt="License"/>
</p>

---

## 🤔 Why ATPspy?

We all run `sudo apt update && sudo apt upgrade -y` blindly. But do you know:

- Is `linux-image` getting updated? (**kernel change = reboot needed**)
- Is `libc6` being upgraded? (**core system library — high risk**)
- Is it just `vim` getting a patch? (**low risk, go ahead**)

**ATPspy** gives you a TUI dashboard to **see, analyze, and selectively upgrade** packages — no more blind updates.

## ✨ Features

| Feature | Status |
|---------|--------|
| 🔒 Password input inside TUI | ✅ |
| 📦 Parse `apt list --upgradable` | ✅ |
| 🔴🟡🟢 Risk-level color coding | 🔄 WIP |
| 📊 Package details (size, version diff) | 🔄 WIP |
| ✅ Select/deselect individual packages | 🔜 Planned |
| 🔄 Live scanning with spinner | 🔜 Planned |
| 📈 Upgrade progress bar | 🔜 Planned |
| 📝 Upgrade logs viewer | 🔜 Planned |

## 🚀 Quick Start

### Prerequisites
- **Linux** with `apt` package manager (Debian/Ubuntu)
- **Rust** 1.75+ ([install](https://rustup.rs/))

### Build & Run

```bash
git clone https://github.com/Parashar-dev/ATPspy.git
cd ATPspy
cargo build --release
cargo run
```

### Development Mode (Mock Data)

Create a `.env` file in the project root:

```env
APP_MODE=development
```

This uses test data from `src/test/mockData/upgradable.txt` instead of real apt commands.

## 🏗️ Project Structure

```
ATPspy/
├── Cargo.toml
├── .env                          # APP_MODE=development|production
├── src/
│   ├── main.rs                   # Entry point + app loop
│   ├── models/
│   │   ├── mod.rs
│   │   └── package.rs            # Package struct, RiskLevel enum
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── password_screen.rs    # 🔒 Password input screen
│   │   ├── list_screen.rs        # 📦 Package list + details
│   │   ├── scan_screen.rs        # 🔄 Scanning animation
│   │   └── upgrad_screen.rs      # 📈 Upgrade progress
│   └── test/
│       └── mockData/
│           └── upgradable.txt    # Sample apt output for testing
└── assets/
    └── banner.png
```

## 🎮 Keybinds

| Key | Action |
|-----|--------|
| `↑` / `k` | Navigate up |
| `↓` / `j` | Navigate down |
| `Enter` | Submit / confirm |
| `Backspace` | Delete character |
| `Esc` / `q` | Quit |

## 🛡️ Risk Levels

| Symbol | Level | Examples |
|--------|-------|----------|
| 🔴 | **Critical** | `linux-image`, `libc6`, `grub-pc` |
| 🔴 | **High** | `systemd`, `dbus`, `libssl` |
| 🟡 | **Medium** | `python3`, security repo packages |
| 🟢 | **Low** | `vim`, `git`, `wget` |

## 🛠️ Tech Stack

- **Language:** Rust 🦀
- **TUI Framework:** [Ratatui](https://github.com/ratatui/ratatui)
- **Terminal Backend:** [Crossterm](https://github.com/crossterm-rs/crossterm)
- **Async Runtime:** [Tokio](https://tokio.rs/)
- **Config:** [dotenvy](https://github.com/allan2/dotenvy)

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/awesome`)
3. Commit your changes (`git commit -m 'Add awesome feature'`)
4. Push to the branch (`git push origin feature/awesome`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

<p align="center">
  Made with 🦀 and ☕ by <a href="https://github.com/Parashar-dev">Parashar</a>
</p>
