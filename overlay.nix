final: prev:
rec {
  rhashes = {
    pkgs = import ./pkgs { pkgs = final; inherit rhashes; };
    rnix-hashes = prev.naersk.buildPackage {
      src = final.builtins.filterSource (path: type: type != "directory" || final.builtins.baseNameOf path != "target") ./.;
      remapPathPrefix = true;
    };
  };
}
