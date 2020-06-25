{ pkgs ?  import ./nix/nixpkgs.nix }:
pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.cargo-edit
    pkgs.gitAndTools.git-extras
    pkgs.mdsh
    pkgs.openssl
    pkgs.pkgconfig
  ]
  ++ pkgs.stdenv.lib.optionals pkgs.stdenv.isDarwin [
    pkgs.darwin.apple_sdk.frameworks.Security
  ]
  ;

  shellHook = ''
    export PATH=$PWD/target/debug:$PATH
  '';
}
