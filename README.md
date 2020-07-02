# rnix-hash: Nix Hash Converter

## Background
In Nix 2.0 CLI, it uses SRI hashes instead of the old format. Meanwhile, some Nix user tends to stay with the old format.

This project trying to bridge the gap between the new SRI format and old format so a happy non-SRI format user can still prefer the old format for their code.


## Installation

nixpkgs-fmt is available in nixpkgs master soon. `nix-env -i rnix-hash`.

It's also possible to install it directly from this repository:

`nix-env -f https://github.com/numtide/rnix-hash/archive/master.tar.gz -i`

## Usage Example

User can provide their hash into `rnix-hash` and by default, it will print out all possible encoding into the terminal.
```
$ rnix-hash sha256-Y39OVtscIh6VSH4WBwCDM/eGPFEOxzXtgnHU708CnqU=

sha256-Y39OVtscIh6VSH4WBwCDM/eGPFEOxzXtgnHU708CnqU=
sha256:19cy097yzm3ihbnkbiqfa4y8dxrkhc00f5ky92aiw8hwvdb4wzv3
sha256:637f4e56db1c221e95487e1607008333f7863c510ec735ed8271d4ef4f029ea5
```

You can also provide `--encoding` options if you want to print out certain encoding.
