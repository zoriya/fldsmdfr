{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    packages = [
      pkgs.cargo
      pkgs.rustfmt
   ];
}

