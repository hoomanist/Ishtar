{ pkgs ? import <nixpkgs> {} }:

let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
in

pkgs.mkShell {
  buildInputs = [
    rust
    pkgs.rust-analyzer
    # keep this line if you use bash
    pkgs.bashInteractive
  ];
}
