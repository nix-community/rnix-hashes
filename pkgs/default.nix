{ rnixHash, pkgs, naersk}:
with pkgs;
rec {
  recurseForDerivations = true;
  rust = callPackage ./rust { };
  naersk = callPackage ./naersk { inherit naersk rust; };
  devEnv = callPackage ./devEnv { name = "rnixHash.pkgs.devEnv"; };
}
