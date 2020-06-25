let
  sources = import ./sources.nix;
  pkgs = import sources.nixpkgs {
    config = { };
    overlays = [
      (self: pkgs: {
        naersk = pkgs.callPackage sources.naersk { };
      })
    ];
  };
in
pkgs
