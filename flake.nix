{
  description = "rnix-hashes";

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
          defaultPackage = pkgs.rhashes.pkgs.devEnv;
          packages = pkgs.rhashes;
          devShell = pkgs.mkShell {
            name = "rnixHash-devShell";
            buildInputs = [ pkgs.rhashes.pkgs.devEnv ];
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
