{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        rust-config =
          (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
        rustc_version = rust-config.toolchain.channel;
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in {
        devShells.default = with pkgs;
          mkShell {
            SSL_CERT_DIR = /etc/ssl/certs;

            buildInputs = [
              openssl_3_3
              pkg-config
              rust-bin.stable.${rustc_version}.default
              rust-analyzer
              cargo
              zstd
            ];

            shellHook = "";
          };
      });
}
