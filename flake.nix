{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = [
            (pkgs.rust-bin.nightly.latest.default.override {
              extensions = [ "rust-analyzer" "clippy" "rust-src" ];
            })
          ];
        };
      }));
}
