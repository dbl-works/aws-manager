# AWS Manager
## Feature Scope
* Generate short-lived password to login to an RDS instance with IAM authentication
* tbd

This is a Rust rewrite based on https://tauri.studio/ to learn Rust.
Join our (internal) Rust-learning channel if you want: [#learning-rust](http://go/rust)

More learning resources:
* [rust-lang/rustlings](https://github.com/rust-lang/rustlings)
* [freecodecamp/rust-in-replit/](https://www.freecodecamp.org/news/rust-in-replit/)
# How to install

Check latest release and install using binary suitable for your OS!
## Project setup

```sh
yarn install
cd src-tauri
cargo install
```

# How to build

```sh
yarn tauri:build
```

on ARM/M1/M2:

```sh
yarn tauri:build --target universal-apple-darwin
```

# How to contribute
* Open an issue to start a conversation
* Open a pull request

### Compiles and hot-reloads for development
```
yarn tauri:serve
```

### Compiles and minifies for production
```
yarn tauri:build
```

### Lints and fixes files
```
yarn lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
