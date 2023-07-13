{
  description = "hibro";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  } @ inputs: let
    # system = "x86_64-linux";
    systems = [
      "aarch64-darwin"
      "aarch64-linux"
      "armv6l-linux"
      "armv7l-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
  in {
    # formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);
    formatter = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in
      pkgs.alejandra);

    devShells = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in rec {
      default = hibro;

      hibro =
        pkgs.mkShell
        {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.rust-analyzer
            pkgs.rustup

            pkgs.pkg-config
            pkgs.openssl
          ];

          shellHook = ''
          '';
        };
    });
  };
}
# {
#   pkgs ? import <nixpkgs> {},
#   extras ? "",
# }:
# pkgs.mkShell {
#   nativeBuildInputs = with pkgs; [
#     rustc
#     cargo
#     rust-analyzer
#     rustup
#
#     pkg-config
#     openssl
#   ];
# }

