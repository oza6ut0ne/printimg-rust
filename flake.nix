{
  description = "Print an image or video in terminal.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, ... }:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import inputs.rust-overlay) ];
        pkgs = import (inputs.nixpkgs) { inherit system overlays; };

        nativeBuildInputs = with pkgs; [
          clang
          pkg-config
          rustPlatform.bindgenHook
        ];

        buildInputs = with pkgs; [ opencv4WithoutCuda ];

        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.stable.latest.minimal;
          rustc = pkgs.rust-bin.stable.latest.minimal;
        };

        GST_PLUGIN_SYSTEM_PATH_1_0 = with pkgs.gst_all_1;
          "${gstreamer.out}/lib/gstreamer-1.0:${gst-plugins-base}/lib/gstreamer-1.0:${gst-plugins-good}/lib/gstreamer-1.0";
      in {
        packages.default = rustPlatform.buildRustPackage {
          inherit buildInputs nativeBuildInputs;
          inherit GST_PLUGIN_SYSTEM_PATH_1_0;

          name = "printimg";
          src = ./.;
          version = self.shortRev or "dev";

          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = true;
          };

          meta.mainProgram = "printi";
        };

        devShells.default = pkgs.mkShell {
          name = "printimg-shell";
          inherit nativeBuildInputs;
          inherit GST_PLUGIN_SYSTEM_PATH_1_0;

          buildInputs = buildInputs ++ (with pkgs.rust-bin; [
            (stable.latest.minimal.override {
              extensions = [ "clippy" "rust-src" ];
            })
            nightly.latest.rustfmt
            nightly.latest.rust-analyzer
          ]);

          RUST_BACKTRACE = 1;
        };
      });
}
