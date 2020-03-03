let 
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in 
  with nixpkgs;

  stdenv.mkDerivation rec {
  name = "env";
  env = buildEnv { name = name; paths = buildInputs; };
  nativeBuildInputs = [ pkgconfig ];
  buildInputs = [
    latest.rustChannels.stable.rust
    entr
  ];
}

