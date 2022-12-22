{ pkgs ? import <nixpkgs> { } }: with pkgs;

stdenv.mkDerivation {
  name = "blurst-env";

  nativeBuildInputs = with buildPackages; [
    rustc
    cargo
    pkgconfig
    (rustfmt.override { asNightly = true; })
    clippy
    crate2nix
  ];

  buildInputs = [
    dbus
  ];

  # Doesn't work in debug builds
  hardeningDisable = [ "fortify" ];

  RUST_BACKTRACE = 1;

  RUST_TOOLCHAIN = buildEnv {
    name = "rust-toolchain";
    paths = with buildPackages; [ rustc cargo ];
  } + "/bin";
  RUST_SRC_PATH = rustPlatform.rustLibSrc;
}
