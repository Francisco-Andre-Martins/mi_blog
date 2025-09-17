{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "mi_blog";
  version = "0.0.0.1";
  cargoLock.lockFile = ./blog/Cargo.lock;
  src = pkgs.lib.cleanSource ./blog;
}
