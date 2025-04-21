{
  description = "Rust flake";

  inputs = {

    nixpkgs.url = "github:nixos/nixpkgs/release-24.11";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    let
      patch-overlay =
        final: prev:
        let
          node-patcher = (
            prevAttrs: {
              patches = prevAttrs.patches ++ [
                (prev.fetchpatch2 {
                  url = "https://github.com/nodejs/node/commit/33f6e1ea296cd20366ab94e666b03899a081af94.patch?full_index=1";
                  hash = "sha256-aVBMcQlhQeviUQpMIfC988jjDB2BgYzlMYsq+w16mzU=";
                })
              ];
            }
          );
        in
        # this creates an attribute set with
        # {
        #   nodejs_19 = prev.nodejs_19.overrideAttrs node-patcher;
        #   [...]
        # }
        prev.lib.genAttrs [
          "nodejs_19"
          "nodejs_20"
          "nodejs_22"
          "nodejs_23"
          "nodejs-slim"
          "nodejs-slim_18"
          "nodejs-slim_22"
          "nodejs-slim_23"
        ] (name: prev."${name}".overrideAttrs node-patcher);
    in
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          patch-overlay
          (import rust-overlay)
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust-select = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
          toolchain.default.override {
            extensions = [
              "rust-analyzer"
              "rust-src"
            ];
          }
        );
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-select
            yarn
          ];
        };
      }
    );

}
