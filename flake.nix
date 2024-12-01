{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, fenix, flake-utils, ... }:
    let supportedSystems = [ "x86_64-linux" "aarch64-darwin" ];
    in flake-utils.lib.eachSystem supportedSystems (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustToolchain = fenix.packages."${system}".stable.withComponents [
          "cargo"
          "rustc"
          "rust-src"
          "rustfmt"
          "clippy"
        ];
        nativeInputs = with pkgs; [
          helix
          nixfmt
          nil
          git
          rustToolchain
          rust-analyzer
        ];
      in {
        devShells.default = pkgs.mkShell { nativeBuildInputs = nativeInputs; };
      });
}

