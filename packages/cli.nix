{
  rustPlatform,
  lib,
  # nativeBuildInputs
  perl,
  ...
}: let
  pname = "stride";
in
  rustPlatform.buildRustPackage {
    version = "0.1.0";
    inherit pname;

    # Nix needs to rebuild a package every time its source changes.
    # Thus we filter out all the files in this repository that are not relevant for the
    # `stride` cli, to avoid having to rebuild it, when a irrelevant file has been
    # changed.
    src = lib.cleanSourceWith {
      # Apply the default source cleaning from nixpkgs
      src = lib.cleanSource ./..;

      # This has been taken from `crane` at:
      # https://github.com/ipetkov/crane/blob/fd86b78f5f35f712c72147427b1eb81a9bd55d0b/lib/filterCargoSources.nix
      filter = orig_path: type: let
        path = builtins.toString orig_path;
        base = builtins.baseNameOf path;
        parentDir = builtins.baseNameOf (builtins.dirOf path);

        matchesSuffix = lib.any (suffix: lib.hasSuffix suffix base) [
          # Keep rust sources
          ".rs"
          # Keep all toml files as they are commonly used to configure other
          # cargo-based tools
          ".toml"
        ];

        # Cargo.toml already captured above
        isCargoFile = base == "Cargo.lock";

        # .cargo/config.toml already captured above
        isCargoConfig = parentDir == ".cargo" && base == "config";
      in
        type == "directory" || matchesSuffix || isCargoFile || isCargoConfig;

      name = pname;
    };

    nativeBuildInputs = [
      # Required by the `openssl` crate
      perl
    ];

    doCheck = true;

    cargoLock = {
      lockFile = ../Cargo.lock;
    };
  }
