{
  description = "Task management tool that synchronizes with git.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    # inputs for following
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs = {};
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    treefmt-nix,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      unfreePkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
        config.allowUnfree = true;
        config.android_sdk.accept_license = true;
      };
      android = unfreePkgs.androidenv.composeAndroidPackages {
        cmdLineToolsVersion = "11.0";
      };

      nightly = false;
      rust =
        if nightly
        then pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)
        else pkgs.rust-bin.stable.latest.default;

      treefmtEval = import ./treefmt.nix {inherit treefmt-nix pkgs;};
    in {
      checks = {
        formatting = treefmtEval.config.build.check self;
      };

      formatter = treefmtEval.config.build.wrapper;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = [
          # Required for building the `openssl-sys` crate, as this uses `perl` to configure
          # `openssl`.
          pkgs.perl
        ];

        env = {
          ANDROID_HOME = "${android.androidsdk}/libexec/android-sdk";
          JAVA_HOME = pkgs.jdk17;
          ANDROID_AVD_HOME = (builtins.toString ./.) + "/.android/avd";
        };

        packages = [
          rust
          pkgs.cargo-edit

          pkgs.flutter
          pkgs.jdk17
          android.platform-tools
        ];
      };
    });
}
# vim: ts=2

