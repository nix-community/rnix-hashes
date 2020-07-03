{ name, buildEnv, pkgs, rnixHash }:
buildEnv {
  inherit name;

  paths = [
    rnixHash.pkgs.rust
    rnixHash.pkgs.naersk
    pkgs.awscli
    pkgs.jq
    pkgs.nixpkgs-fmt
  ];
}
