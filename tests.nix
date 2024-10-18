{
  pkgs,
  lib,
  specialArgs,
}: let
  # for `nix eval --file` (as it does not support args) use:
  # ```
  # specialArgs = {};
  # pkgs = (builtins.getFlake "nixpkgs").legacyPackages."x86_64-linux";
  # inherit (pkgs) lib;
  # ```
  # instead of the function arguments above.
  importTests' = test: let
    basename = builtins.baseNameOf test;
    testName = builtins.baseNameOf (lib.strings.removeSuffix "/${basename}" "${builtins.toString test}");
  in {
    name = "${testName}";
    value = pkgs.callPackage test specialArgs;
  };

  importTests = dir:
    builtins.listToAttrs (builtins.map importTests' (
      lib.fileset.toList (lib.fileset.fileFilter (file: file.name == "test.nix") dir)
    ));
in
  importTests ./tests
