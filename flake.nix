{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-parts.url = "github:hercules-ci/flake-parts";
    bevy_cli.url = "github:TheBevyFlock/bevy_cli";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      rust-overlay,
      flake-parts,
      bevy_cli,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = ["aarch64-darwin"];
      perSystem = { system, pkgs, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            (rust-bin.stable.latest.default.override {
              extensions = [
                "rust-analyzer"
              ];
            })
            bacon
          ];
        };
      };
    };
}
