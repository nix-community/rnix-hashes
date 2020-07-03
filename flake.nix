{
  description = "rnix-hash";

  inputs.nixpkgs.url = github:NixOS/nixpkgs/master;
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.naersk = {
    type = "github";
    owner = "nmattia";
    repo = "naersk";
    flake = false;
  };
  inputs.mozilla-overlay = {
    type = "github";
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
    flake = false;
  };

  outputs = { self, nixpkgs, naersk, mozilla-overlay, flake-utils }:
    {
      overlay = import ./overlay.nix {inherit naersk;};
    }
    //
    (
      flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              (import mozilla-overlay)
              self.overlay
            ];
          };
        in
        {
          defaultPackage = pkgs.rnixHash.pkgs.devEnv;
          packages = pkgs.rnixHash;
          devShell = pkgs.mkShell {
            name = "rnixHash-devShell";
            buildInputs = [ pkgs.rnixHash.pkgs.devEnv ];
            shellHook =
              ''
                PATH=$prefix/bin:$PATH
                unset PYTHONPATH
              '';
          };
        }
      )
    );
}
