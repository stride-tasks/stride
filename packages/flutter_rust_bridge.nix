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
  version = "2.9.0";

  src = fetchFromGitHub {
    owner = "fzyzcjy";
    repo = "flutter_rust_bridge";
    rev = "v${version}";
    hash = "sha256-SGbl1l2jtANO9QMNz9GDXI794RY/K+ldpmDxLkqAa+Y=";
    fetchSubmodules = true;
  };

  cargoHash = "sha256-W6iHdLkQ/d6xqpQLf63I+97MXn3blLZw1IDxk4S1uTo=";

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
