{
  description = "rp2040-pac crate";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };
  outputs =
    {
      nixpkgs,
      rust-overlay,
      treefmt-nix,
      utils,
      ...
    }:
    (utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        formatter = (import treefmt-nix).mkWrapper pkgs {

          # Used to find the project root
          projectRootFile = ".git/config";
          # Enable the terraform formatter
          programs = {
            rustfmt.enable = true;
            taplo.enable = true;
            toml-sort.enable = true;
            nixfmt.enable = true;
          };
        };
      in
      {
        inherit formatter;
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rust-bin.stable.latest.default
          ];
          shellHook = ''
            echo "To build the project, use 'cargo build --target armv6m-none-eabi'"
            echo "To flash the project, use 'cargo run --target armv6m-none-eabi'"
          '';
        };
      }
    ));
}
