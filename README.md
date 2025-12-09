# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## compile

```bash
yarn tauri dev
```

## macos compile exe

```bash
# yarn tauri build -- --target x86_64-pc-windows-msvc
yarn tauri build --target x86_64-pc-windows-gnu --no-bundle
```