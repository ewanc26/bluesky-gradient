{
  description = "bluesky-gradient — sky colour gradient image generator";

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
            packages = with pkgs; [
              python3
              python3Packages.pip
              python3Packages.virtualenv
              python3Packages.pillow
              python3Packages.numpy
            ];

            shellHook = ''
              echo "bluesky-gradient dev shell ready (Python 3 + Pillow + NumPy)"
            '';
          };
        }
      );

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
    };
}
