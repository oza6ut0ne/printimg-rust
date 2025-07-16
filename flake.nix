{
  description = "Print an image or video in terminal.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nix-appimage = {
      url = "github:ralismark/nix-appimage";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ self, ... }:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem =
        { system, ... }:
        let
          name = "printimg";
          pname = name;
          binname = "printi";
          overlays = [ (import inputs.rust-overlay) ];
          pkgs = import inputs.nixpkgs { inherit system overlays; };

          nativeBuildInputs = with pkgs; [
            clang
            pkg-config
            rustPlatform.bindgenHook
            libllvm
            makeWrapper
          ];

          buildInputs = with pkgs; [ opencv4WithoutCuda ];

          rustPlatform = pkgs.makeRustPlatform {
            cargo = pkgs.rust-bin.stable.latest.minimal;
            rustc = pkgs.rust-bin.stable.latest.minimal;
          };

          GST_PLUGIN_SYSTEM_PATH_1_0 =
            with pkgs.gst_all_1;
            "${gstreamer.out}/lib/gstreamer-1.0:${gst-plugins-base}/lib/gstreamer-1.0:${gst-plugins-good}/lib/gstreamer-1.0";
        in
        {
          packages = rec {
            default = bin;

            bin = rustPlatform.buildRustPackage {
              inherit name pname;
              inherit buildInputs nativeBuildInputs;
              inherit GST_PLUGIN_SYSTEM_PATH_1_0;

              src = ./.;
              version = self.shortRev or self.dirtyShortRev or "dev";
              meta.mainProgram = binname;

              cargoLock = {
                lockFile = ./Cargo.lock;
                allowBuiltinFetchGit = true;
              };

              postFixup = ''
                llvm-strip $out/bin/${binname}
                wrapProgram $out/bin/${binname} \
                  --prefix GST_PLUGIN_SYSTEM_PATH_1_0 : ${GST_PLUGIN_SYSTEM_PATH_1_0}
              '';
            };

            appimage = inputs.nix-appimage.lib.${system}.mkAppImage {
              program = "${bin.out}/bin/${binname}";
            };

            docker = pkgs.dockerTools.buildLayeredImage {
              name = binname;
              tag = "latest";
              contents = [ bin ];
              config = {
                WorkingDir = "/workdir";
                Entrypoint = "/bin/${binname}";
              };
            };
          };

          devShells.default = pkgs.mkShell {
            inherit name;
            inherit nativeBuildInputs;
            inherit GST_PLUGIN_SYSTEM_PATH_1_0;

            buildInputs =
              buildInputs
              ++ (with pkgs.rust-bin; [
                (stable.latest.minimal.override {
                  extensions = [
                    "clippy"
                    "rust-src"
                  ];
                })
                nightly.latest.rustfmt
                nightly.latest.rust-analyzer
              ]);

            RUST_BACKTRACE = 1;
          };
        };
    };
}
