{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        dependencies = with pkgs; [ openssl ];
        toolchain = with pkgs; [ cargo rust-analyzer rustc rustfmt pre-commit rustPackages.clippy ];
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          buildInputs = dependencies;
        };
        devShell = with pkgs; mkShell {
          buildInputs = dependencies ++ toolchain;
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      }
    );
}
