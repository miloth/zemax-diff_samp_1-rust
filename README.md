# Zemax Diffraction DLL Example in Rust! 🦀

This can be modified or extended to include the other kinds of DLLs supported by Zemax.

## Compilation

### On Windows

The code can be compiled to a DLL using the following command:

```sh
cargo build --release
```

and the resulting DLL will be in the `./target/release/` folder, ready to be copied in the Zemax DLL folder.

### On Linux or macOS

You need to have the following pre-requisites:

- The `mingw-w64` toolchain installed, which can be obtained from the package manager of your distribution on Linux or from [homebrew](https://brew.sh/) on macOS, here a few examples:

  ```sh
  # On Ubuntu
  sudo apt install mingw-w64
  # On macOS
  brew install mingw-w64
  ```

- The `x86_64-pc-windows-gnu` target installed in your Rust toolchain, which can be done using the following command:

  ```sh
  rustup target add x86_64-pc-windows-gnu
  ```

Then, you can compile the DLL using the following command:

```sh
cargo build --release --target=x86_64-pc-windows-gnu
```

and the resulting DLL will be in the `./target/x86_64-pc-windows-gnu/release/` folder, ready to be copied in the Zemax DLL folder.

## Docs Generation

Simply run:

```sh
cargo doc --document-private-items --no-deps  --target=x86_64-pc-windows-gnu
```

Add the additional `--open` flag to open the documentation in your default web browser.
