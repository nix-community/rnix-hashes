{ rnixHash, pkgs }:
with pkgs;
rec {
  devEnv = callPackage ./devEnv { name = "rnixHash.pkgs.devEnv"; };
}
