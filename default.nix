{ pkgs, nixpkgs, system, makeRustPlatform, rust-overlay }:
let
  rustPkgs = import nixpkgs {
    inherit system;
    overlays = [ (import rust-overlay) ];
  };

  rustVersion = "1.91.0";

  wasm = "wasm32-unknown-unknown";
  wasi = "wasm32-wasip2";

  rustDefaultTarget = rustPkgs.rust-bin.stable.${rustVersion}.default;

  rustWithWasmTarget = rustPkgs.rust-bin.nightly.${rustVersion}.default.override {
    targets = [ wasm ];
  };

  rustPlatform = makeRustPlatform {
    cargo = rustDefaultTarget;
    rustc = rustDefaultTarget;
  };

  rustPlatformWasm = makeRustPlatform {
    cargo = rustWithWasmTarget;
    rustc = rustWithWasmTarget;
  };

  common = {
    version = "0.1.6";
    src = self;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [ pkgs.pkg-config ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
in {
  workspace = pkgs.rustPlatformWasm.buildRustPackage (common // {
    cargoBuildFlags = "--release --workspace";
  });
}