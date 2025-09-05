{
  description = "Slappy";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "slappy";
          version = "0.1.0";
          buildInputs = [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.libGL
          ];

          CARGO_INCREMENTAL = 0;
          src = ./.;
          doCheck = false;
          cargoLock.lockFile = ./Cargo.lock;
        };

        devShells.default = pkgs.mkShell {
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.libGL
          ];
        };
      }
    );
}
