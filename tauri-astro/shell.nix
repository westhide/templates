{
  pkgs ? import <nixpkgs> {
    config = {
      allowUnfree = true;
      android_sdk.accept_license = true;
    };
  },
  ANDROID_HOME ? "$HOME/Android/Sdk",
}:

with pkgs;
let
  AndroidPkgs = pkgs.androidenv.composeAndroidPackages {
    includeNDK = true;
    includeEmulator = true;
    includeSystemImages = true;

    buildToolsVersions = [
      "latest"
    ];
    platformVersions = [ "36" ];
  };
in
mkShell {
  shellHook = ''
    export ANDROID_SDK_ROOT="${AndroidPkgs.androidsdk}/libexec/android-sdk";
    export ANDROID_NDK_ROOT="$ANDROID_SDK_ROOT/ndk-bundle";

    export ANDROID_HOME="${ANDROID_HOME}";
    export NDK_HOME="$ANDROID_NDK_ROOT";
    export WEBKIT_DISABLE_DMABUF_RENDERER=1;
  '';

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
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
