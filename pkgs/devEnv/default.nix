{ name, buildEnv, pkgs, rhashes }:
buildEnv {
  inherit name;

  paths = [
    pkgs.awscli
    pkgs.cargo
    pkgs.cargo-edit
    pkgs.cargo.passthru.rustc
    pkgs.jq
    pkgs.nixpkgs-fmt
    pkgs.rustc
  ];
}
