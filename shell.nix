{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    gobject-introspection
    cargo
    cargo-tauri
    nodejs
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
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
