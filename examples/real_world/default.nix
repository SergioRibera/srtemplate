let
  inherit
    (builtins)
    currentSystem
    fromJSON
    readFile
    ;
  getFlake = name:
    with (fromJSON (readFile ../flake.lock)).nodes.${name}.locked; {
      inherit rev;
      outPath = fetchTarball {
        url = "https://github.com/${owner}/${repo}/archive/${rev}.tar.gz";
        sha256 = narHash;
      };
    };
in
{ system ? currentSystem
, pkgs ? import (getFlake "nixpkgs") { localSystem = { inherit system; }; }
, crane
, cranix
, fenix
, ...
}:
let
  # fenix: rustup replacement for reproducible builds
  toolchain = fenix.${system}.fromToolchainFile {
    file = ./rust-toolchain.toml;
    sha256 = "sha256-Ngiz76YP4HTY75GGdH2P+APE/DEIx2R/Dn+BwwOyzZU=";
  };
  # crane: cargo and artifacts manager
  craneLib = crane.${system}.overrideToolchain toolchain;
  # cranix: extends crane building system with workspace bin building and Mold + Cranelift integrations
  cranixLib = craneLib.overrideScope' (cranix.${system}.craneOverride);
in
{
  # `nix develop`
  devShells.default = cranixLib.devShell {
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
