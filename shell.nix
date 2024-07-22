{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    rustup
    wasm-pack
    cargo-generate
    nodejs
    openssl
  ];

  shellHook = ''
    # Set up Rust toolchain and target for WebAssembly
    rustup default stable
    rustup target add wasm32-unknown-unknown
    export NODE_OPTIONS=--openssl-legacy-provider
    echo "Environment set up for Rust WebAssembly development with wasm-pack, cargo-generate, and npm."
  '';
}
