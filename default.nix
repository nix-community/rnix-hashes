{ pkgs ? import <nixpkgs> { } }:
let
  naersk-src = pkgs.fetchgit {
    url = "https://github.com/nmattia/naersk.git";
    rev = "20ec73e49b9d40928b97014a40dac2cf28c56bee";
    sha256 = "CMSFTDgaW/rs53zXcGKZNjXuXX8anCrNh101yGYvNlw=";
  };

  naersk = pkgs.callPackage naersk-src { };

  lib = pkgs.lib;

  # another attempt to make filterSource nicer to use
  allowSource = { allow, src }:
    let
      out = builtins.filterSource filter src;
      filter = path: _fileType:
        lib.any (checkElem path) allow;
      checkElem = path: elem:
        lib.hasPrefix (toString elem) (toString path);
    in
    out;
  src = allowSource {
    allow = [
      ./Cargo.lock
      ./Cargo.toml
      ./fuzz
      ./src
      ./test_data
      ./wasm
    ];
    src = ./.;
  };
in
naersk.buildPackage {
  inherit src;
  root = ./.;
  cratePaths = [ "." ];
}
