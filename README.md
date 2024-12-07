## File Structure

```
|-- Rust_Project_Init
      |-- .gitignore
      |-- LICENSE
      |-- README.md
      |-- Frontend     # Frontend Code
          |-- .DS_Store
          |-- .gitignore
          |-- Cargo.lock
          |-- Cargo.toml
          |-- README.md
          |-- index.html
          |-- trunk.toml
          |-- src           # Source Code Folder
          |   |-- api.rs    # API for backend (to be developed)
          |   |-- lib.rs
          |   |-- main.rs
          |   |-- components
          |   |   |-- header.rs        # All Page Header
          |   |   |-- mod.rs
          |   |   |-- sidebar          # SideBar Code
          |   |       |-- history.rs   # SideBar Component History
          |   |       |-- mod.rs
          |   |       |-- sidebar.rs   # SideBar Component SideBar
          |   |       |-- steps.rs     # SideBar Component Steps
          |   |-- pages                # SideBar Component Steps
          |       |-- mod.rs
          |       |-- first_step_page             # Code of the “First Steps” page
          |       |   |-- first_step_page.rs
          |       |   |-- mod.rs
          |       |-- home_page                   # Code of the “Home” page
          |           |-- content_section.rs
          |           |-- mod.rs
          |-- static                   # Other Resources
          |   |-- ShapeC.png
          |   |-- down.png
          |   |-- history.png
          |   |-- logo.png
          |   |-- more.png
          |   |-- sample.png
          |   |-- setting.png
          |   |-- star.png
          |   |-- steps.png
          |   |-- styles.css
          |-- |-- up.png
```

## Installation

```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
```

## Others

目前前后端是分开的，后端在http://127.0.0.1:8081，前端在http://127.0.0.1:8080
前端如何运行：trunk serve
后端： cargo run
前端用的是 Yew，格式/Tut 参考：https://yew.rs/docs/getting-started/introduction

## License

[MIT](https://choosealicense.com/licenses/mit/)
