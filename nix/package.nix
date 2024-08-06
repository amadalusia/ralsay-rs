{
  lib,
  rustPlatform,
  stdenvAdapters,
  llvm,
}: let
  toml = (lib.importTOML ../Cargo.toml).package;
  pname = toml.name;
  inherit (toml) version;
in
  rustPlatform.buildRustPackage.override {stdenv = stdenvAdapters.useMoldLinker llvm.stdenv;} {
    RUSTFLAGS = "-C link-arg=-fuse-ld=mold";

    inherit pname version;

    src = builtins.path {
      name = "${pname}-${version}";
      path = lib.sources.cleanSource ../.;
    };

    cargoLock.lockFile = ../Cargo.lock;
  }
