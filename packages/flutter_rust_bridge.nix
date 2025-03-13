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

  # See: https://nixos.org/manual/nixos/unstable/release-notes.html#sec-nixpkgs-release-25.05
  useFetchCargoVendor = true;

  src = fetchFromGitHub {
    owner = "fzyzcjy";
    repo = "flutter_rust_bridge";
    rev = "v${version}";
    hash = "sha256-pvKCiv7hUgetTXXp+NCs04Qo9xWaLUE2T1yHENhTGl4=";
    fetchSubmodules = true;
  };

  cargoHash = "sha256-efMA8VJaQlqClAmjJ3zIYLUfnuj62vEIBKsz0l3CWxA=";

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
