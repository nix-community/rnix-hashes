# rnix-hashes: Nix hash converter

Newer version of Nix output hash mismatches in the SRI format by default. But
sometimes you want to access the same hashes in different encodings.

This is a small utility that allows you to do just that and convert between
the formats.

## Example

`$ rnix-hashes sha256-Y39OVtscIh6VSH4WBwCDM/eGPFEOxzXtgnHU708CnqU=`
```
SRI	sha256-Y39OVtscIh6VSH4WBwCDM/eGPFEOxzXtgnHU708CnqU=
base16	637f4e56db1c221e95487e1607008333f7863c510ec735ed8271d4ef4f029ea5
base32	19cy097yzm3ihbnkbiqfa4y8dxrkhc00f5ky92aiw8hwvdb4wzv3
base64	Y39OVtscIh6VSH4WBwCDM/eGPFEOxzXtgnHU708CnqU=

```

## Usage

`$ rnix-hashes --help`
```
rnix-hashes 0.2.0
Andika Demas Riyandi <andika.riyan@gmail.com>
Nix hash converter

USAGE:
    rnix-hashes [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --encoding <ENCODING>    Sets specific encoding: BASE16|BASE32|BASE64|PBASE16|PBASE32|PBASE64|SRI

ARGS:
    <INPUT>    Sets the input file to use
```

## Installation

rnix-hashes will be available in nixpkgs master soon. `nix-env -i rnix-hashes`.

It's also possible to install it directly from this repository:

 `nix-env -f https://github.com/numtide/rnix-hashes/archive/master.tar.gz -i`

## License

MIT, Copyright 2020 NumTide Ltd and contributors
