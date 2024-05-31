{
  description = "Scripts for eww";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];

      perSystem = {
        config,
        pkgs,
        system,
        ...
      }: let
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            clippy
            rust-analyzer
            rustc
            rustfmt
            rustPackages.clippy
          ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes."hyprland-0.4.0-alpha.2" = "sha256-o0W6rNyXGZ9X1hy4Obp6qH+D/hd8SklM0Veixl0YxcY=";
          };
          src = pkgs.lib.cleanSource ./.;
        };
      };
    };
}
