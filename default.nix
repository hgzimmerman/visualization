with import <nixpkgs> {};


stdenv.mkDerivation rec {
  name = "Yeet";
  env = buildEnv { name = name; paths = buildInputs; };
  buildInputs = [
    alsaLib
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    vulkan-tools
    vulkan-loader
    vulkan-headers
    openssl # Lib for TLS integration needed for Diesel + Warp
    pkgconfig # resolve dependencies
  ];
  LD_LIBRARY_PATH="${vulkan-loader}/lib:${vulkan-tools}/lib";
  shellHook = ''
    alias docs='cargo rustdoc --bins --open -- --document-private-items'
  '';
}
