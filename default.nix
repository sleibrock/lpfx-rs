{ nixpkgs ? import <nixpkgs> { } }:

let
  pkgs = [
    nixpkgs.portmidi
  ];

in
  nixpkgs.stdenv.mkDerivation {
    name = "env";
    buildInputs = pkgs;
  }
