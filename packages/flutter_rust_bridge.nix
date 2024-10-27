{
  lib,
  rustPlatform,
  fetchFromGitHub,
  stdenv,
  makeWrapper,
  darwin,
  cargo-expand,
}:
rustPlatform.buildRustPackage rec {
  pname = "flutter-rust-bridge";
  version = "2.5.0";

  src = fetchFromGitHub {
    owner = "fzyzcjy";
    repo = "flutter_rust_bridge";
    rev = "v${version}";
    hash = "sha256-Te0egYpX7dWrXoeraNpNbNUxVBc6o2wFxotil1tHnzw=";
    fetchSubmodules = true;
  };

  cargoHash = "sha256-kCxoIwPkCZFn5J/D7AhvolAG+qhsbliNMD4pJ7spHwg=";

  buildInputs = lib.optionals stdenv.hostPlatform.isDarwin [
    darwin.apple_sdk.frameworks.CoreFoundation
    darwin.apple_sdk.frameworks.CoreServices
  ];

  nativeBuildInputs = [
    makeWrapper
  ];
  postInstall = ''
    wrapProgram $out/bin/flutter_rust_bridge_codegen \
      --prefix PATH : ${lib.makeBinPath [cargo-expand]}
  '';

  # The tests seem to fail in the build sandbox.
  doCheck = false;

  meta = {
    description = "Flutter/Dart <-> Rust binding generator, feature-rich, but seamless and simple";
    homepage = "https://github.com/fzyzcjy/flutter_rust_bridge";
    changelog = "https://github.com/fzyzcjy/flutter_rust_bridge/blob/${src.rev}/CHANGELOG.md";
    license = lib.licenses.mit;
    mainProgram = "flutter-rust-bridge";
  };
}
