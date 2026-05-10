# 🦀 Rust Interactive Terminal Portfolio

A high-performance, type-safe interactive terminal portfolio built entirely in **Rust** using **WebAssembly (WASM)** and the **Yew** framework.

![Rust](https://img.shields.io/badge/language-rust-orange.svg)
![WASM](https://img.shields.io/badge/target-wasm-blue.svg)
![Yew](https://img.shields.io/badge/framework-yew-green.svg)
![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)

## 🚀 Why Rust for a Portfolio?

As an ECE Engineer specializing in **Systems Programming** and **AI/ML**, I chose Rust for this project to demonstrate:
- **Memory Safety & Type Security**: The entire UI state is managed by Rust's strict ownership model, preventing common runtime errors found in JavaScript.
- **Performance**: High-speed command parsing and rendering via WebAssembly.
- **Novelty**: Moving beyond standard web stacks to leverage the future of web performance.

## 🏗️ Architecture: The 4-Pillar System

The project follows a modular 4-pillar architectural pattern:
1. **Data Modeling**: Strong typing for Projects, Experience, and Research using Rust `structs` and `enums`.
2. **State Management**: Reactive UI updates handled by Yew's `use_state` hooks.
3. **The "Brain" (Command Engine)**: A custom-built parsing engine that maps user input to typed commands using Pattern Matching.
4. **The UI (Rendering)**: A functional component model using the `html!` macro for high-performance DOM updates.

## 💻 Available Commands

Type these in the terminal to interact:
- `help`: See all available commands.
- `about`: A brief bio and technical background.
- `projects`: Detailed list of my high-performance C/C++ and Rust projects.
- `research`: Publications and research work (Deep Learning, DSP, Security).
- `experience`: Professional experience and internships.
- `skills`: Technical stack across Languages, AI/ML, and Tools.
- `clear`: Reset the terminal screen.

## 🛠️ Local Development

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Trunk](https://trunkrs.dev/#install)
- WASM Target: `rustup target add wasm32-unknown-unknown`

### Running Locally
```bash
# Clone the repository
git clone https://github.com/hitesh-bhatnagar/Terminal-Portfolio.git

# Navigate to the folder
cd Terminal-Portfolio

# Start the development server
trunk serve
```

## 🌐 Deployment & CI/CD

This project is deployed using **GitHub Actions**. Every push to the `main` branch triggers an automated build pipeline that:
1. Sets up the Rust environment.
2. Installs the WASM target and Trunk.
3. Builds the production WASM bundle.
4. Deploys the static assets to GitHub Pages.

---
Created by [Hitesh Bhatnagar](https://github.com/hitesh-bhatnagar)
