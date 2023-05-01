{
  pkgs ? import <nixpkgs> {},
  extras ? "",
}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    rust-analyzer
    rustup

    pkg-config
    openssl
  ];
}
