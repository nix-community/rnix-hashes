final: prev:
rec {
  rnixHash = {
    pkgs = import ./pkgs { pkgs = final; inherit rnixHash; };
    rnix-hash = prev.naersk.buildPackage {
      src = final.builtins.filterSource (path: type: type != "directory" || final.builtins.baseNameOf path != "target") ./.;
      remapPathPrefix = true;
    };
  };
}
