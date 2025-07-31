{
  pkgs ? import <nixpkgs> { },
  ANDROID_HOME ? "$HOME/Android/Sdk",
}:
pkgs.mkShell {
  shellHook = ''
    export ANDROID_HOME=${ANDROID_HOME}
    export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"
    export WEBKIT_DISABLE_DMABUF_RENDERER=1
  '';

  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    nodejs
  ];

  buildInputs = with pkgs; [
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
}
