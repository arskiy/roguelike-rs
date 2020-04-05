{
    pkgs   ? import <nixpkgs> {},
    stdenv ? pkgs.stdenv
}:
rec {
  roguelike = stdenv.mkDerivation {
    name = "roguelike-rs";
    version = "0.1.0";
    buildInputs = with pkgs; [
      ncurses
      pkg-config
      binutils
    ];
  };
}
