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
        runtime_dep = pkgs.lib.makeLibraryPath [
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.libGL
          pkgs.fontconfig.lib
        ];
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          name = "slappy";
          version = "0.1.0";
          buildInputs = [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.libGL
          ];
          nativeBuildInputs = [ pkgs.patchelf ];
          CARGO_INCREMENTAL = 0;
          src = ./.;
          doCheck = false;
          cargoLock.lockFile = ./Cargo.lock;
          postFixup = ''
            patchelf --add-rpath "${runtime_dep}" "$out/bin/${name}"
          '';
        };

        devShells.default = pkgs.mkShell {
          LD_LIBRARY_PATH = runtime_dep;
        };
      }
    );
}
