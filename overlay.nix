{ naersk }:
final: prev:
rec {
  rnixHash = {
    pkgs = import ./pkgs { pkgs = final; inherit rnixHash naersk; };
    rnix-hash = naersk.buildPackage {
      src = final.builtins.filterSource (path: type: type != "directory" || final.builtins.baseNameOf path != "target") ./.;
      remapPathPrefix = true;
    };
  };
}
