{
  description =
    "A developmental environment for a Rust project using Nix flakes with direnv";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in rec {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "rstm";
          version = "0.1.6";
          src = self; # ./.;
          cargoLock = { lockFile = ./Cargo.lock; };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.pkg-config
            pkgs.openssl
            pkgs.cargo
            pkgs.rust-analyzer
            pkgs.cargo-binstall
            pkgs.cargo-criterion
          ];
          shellHook = ''
            export CARGO_HOME=$PWD/.cargo
          '';
        };
      });
}
