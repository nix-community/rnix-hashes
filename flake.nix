{
  description = "rnix-hash";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.naersk.url = "github:zimbatm/naersk";

  outputs = { self, nixpkgs, naersk, flake-utils }:
    {
      overlay = import ./overlay.nix;
    }
    //
    (
      flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              naersk.overlay
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
