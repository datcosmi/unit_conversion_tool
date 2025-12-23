# Unit Conversion Helper CLI

A small and friendly **CLI conversion tool written in Rust** 🦀  
Currently supports only **temperature conversions**, with plans to expand to **any kind of unit conversion** in the future.

> ⚠️ **Disclaimer**  
> This project is mainly a **learning exercise** to practice Rust fundamentals (enums, structs, traits, input handling, testing, etc.).  

## 🚀 Planned Features

- Support for more conversion types:
  - Length (meters, kilometers, miles…)
  - Weight (grams, kilograms, pounds…)
  - Time, speed, etc.
- Better modularization as the project grows

## 🧑‍💻 Requirements

- **Rust** (latest stable recommended)
- A terminal that supports ANSI colors

## 🦀 Installing Rust

If you don’t have Rust installed yet, the easiest way is via **rustup**.

### Linux / macOS
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Windows

Download and run the installer from [the official website] (https://www.rust-lang.org/tools/install).

After installation, verify it worked:
```bash
rustc --version
cargo --version
```

## 📦 Cloning the Project
```bash
git clone https://github.com/datcosmi/unit_helper.git
cd unit_helper
```

## ▶️ Running the Tool

Use Cargo to build and run the project:
```bash
cargo run
```

## 📜 License
MIT -- feel free to use, learn from, or modify this project.
