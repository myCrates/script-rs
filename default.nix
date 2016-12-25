with import <nixpkgs> {}; {
  rustEnv = stdenv.mkDerivation {
    name = "script-rs";
    buildInputs = [ stdenv rustc cargo ];
    shellHook =
      ''
      '';
  };
}
