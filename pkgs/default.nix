{ rhashes, pkgs }:
with pkgs;
rec {
  devEnv = callPackage ./devEnv { name = "rnix-hashes.pkgs.devEnv"; };
}
