let
  inherit (builtins) currentSystem;
in
{ system ? currentSystem
, pkgs
, crane
, fenix
, ...
}:
let
  # fenix: rustup replacement for reproducible builds
  toolchain = fenix.${system}.fromToolchainFile {
    file = ./rust-toolchain.toml;
    sha256 = "sha256-s1RPtyvDGJaX/BisLT+ifVfuhDT1nZkZ1NcK8sbwELM=";
  };
  # crane: cargo and artifacts manager
  craneLib = crane.overrideToolchain toolchain;
  # cranix: extends crane building system with workspace bin building and Mold + Cranelift integrations
in
{
  # `nix develop`
  devShells.default = craneLib.devShell {
    packages = with pkgs;
      [
        nodejs
        toolchain
        pkg-config
        sass
        trunk
        leptosfmt
        cargo-make
        cargo-release
        binaryen
      ];
  };
}
