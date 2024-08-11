{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    udev
    alsa-lib
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr # To use the x11 feature
    xorg.libxcb
    libxkbcommon
    wayland # To use the wayland feature

    protobuf_27
    protoc-gen-go
    protoc-gen-go-grpc
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
