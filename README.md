# 📐 Derivative Calculator

A powerful symbolic derivative calculator built in **Rust**, featuring both a **command-line interface (CLI)** and a modern **desktop UI** using **Tauri**.


## ✨ Features

- 🖥️ Cross-platform **Tauri UI**
- 🔧 Fast and efficient **CLI tool**
- 🧠 Symbolic differentiation with rich syntax support
- 🧮 Mathematical expressions parsing and evaluation



## 🧠 Supported Syntax

```
- sin(x)         : Sine of x
- cos(x)         : Cosine of x
- tan(x)         : Tangent of x
- sinh(x)        : Hyperbolic sine
- cosh(x)        : Hyperbolic cosine
- tanh(x)        : Hyperbolic tangent
- asin(x)        : Inverse sine (arcsin)
- acos(x)        : Inverse cosine (arccos)
- atan(x)        : Inverse tangent (arctan)
- exp(x)         : Exponential (e^x)
- log(x)         : Natural logarithm (ln)
- sin^2(x)       : (sin(x))^2
- cos^3 x        : (cos(x))^3
- 2sinx          : 2 * sin(x)
- x^3 + 2x + 1   : Polynomial expressions
-sin(x)+sin^(2x)(x)+2x^x :For complexly chained problems as well
- x^x            : x raised to the power of x (variable exponent)
- sin^x(x)       : (sin(x))^x (function raised to a variable power) This can be done for any function
- (x+1)*(x-1)    : Parentheses grouping
- pi             : π ≈ 3.14159
- e              : Euler's number ≈ 2.71828
- deg            : π/180
- exit           : Exit CLI
```

### 💡 Tips

- Use implicit multiplication: `2x` = `2*x`, `sin2x` = `sin(2*x)`
- Use powers: `sin^2(x)` = `(sin(x))^2`
- Use parentheses for clarity: `sin^2(x+1)`
- Supported functions: `sin`, `cos`, `tan`, `exp`, `log`, `sinh`, `cosh`, `tanh`, etc.

---

## 🖥️ Tauri UI

### Run the UI

```bash
cd derivative_ui
npm install
npm run tauri dev
```

---

## 🧪 CLI Tool

### Run the CLI

```bash
cargo run
```

Example:

```text
> Enter expression: sin^2(x)
Derivative: 2*sin(x)*cos(x)
```

---

## 📁 Project Structure

```
.
├── derivative_ui/     # Tauri UI
├── src/               # CLI source (Rust)
├── screenshots/       # UI screenshots and images
├── target/
├── Cargo.toml
├── Cargo.lock
└── .gitignore
```

---

## 🛠 Installation

### Requirements

- [Rust](https://www.rust-lang.org/)
- [Node.js & npm](https://nodejs.org/) for the Tauri UI

### Install CLI Globally

```bash
cargo install --path .
```

---
---

## 🖼️ UI Screenshot

![Derivative Calculator UI - Main Interface](screenshots/ui1.jpg)
---
## 📜 License

Licensed under the MIT License.