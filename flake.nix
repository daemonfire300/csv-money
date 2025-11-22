{
  description = "csv-money flake (mildly unpolished)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  # TODO(juf): Add crane for
  #   1. Building
  #   2. Running tests
  #   3. Clippy/fmt/sec-adv db

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in {

        devShells.default = let
          stable-rust = pkgs.rust-bin.stable.latest;
          rust-core = stable-rust.default;
          stable-rust-analyzer = stable-rust.rust-analyzer;
        in with pkgs;
        mkShell {
          buildInputs = [
            openssl # for convenience, using rusttls is fine too
            pkg-config
            eza # optional
            fd # optional
            rust-core
            stable-rust-analyzer
            cargo-expand
          ];

          # TODO(juf): using exec $SHELL does not really work. nix-shell -c $SHELL works well. Need to dig into why that is the case
          shellHook = ''
            echo "Entering system-specific shell <$SHELL>"
            #exec $SHELL (not properly working, you have to follow the description from the flake.nix and use nix-shell -c $SHELL
          '';
        };
      });
}
