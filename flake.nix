{
  description = "bluesky-gradient — Gradient image generator for Bluesky (Rust)";

  # Pin to nixos-25.11 for a stable toolchain; the crate has no platform-
  # specific dependencies so any channel works.
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in {
      devShells = forAllSystems (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        {
          default = pkgs.mkShell {
            # Core Rust toolchain plus libfontconfig for ab_glyph rasterisation.
            # openssl is included for cargo-native dependencies (not used directly
            # by this crate but commonly needed).
            packages = with pkgs; [
              rustc
              cargo
              rust-analyzer
              pkg-config
              openssl
              fontconfig
            ];

            shellHook = ''
              echo "bluesky-gradient dev shell ready (Rust)"
            '';
          };
        }
      );

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
    };
}
