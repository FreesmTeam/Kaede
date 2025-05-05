{
  description = "An Kaede dev env";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    unstable.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      unstable,
      flake-utils,
      ...
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            bun
            cargo
            rustc
            pkg-config
            nodejs
            webkitgtk
            libsoup
            gtk3
            cairo
            pango
            atk
            gdk-pixbuf
            glib
            openssl
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="${
              pkgs.lib.makeLibraryPath [
                pkgs.webkitgtk
                pkgs.gtk3
                pkgs.glib
                pkgs.libsoup
                pkgs.cairo
                pkgs.pango
                pkgs.atk
                pkgs.gdk-pixbuf
              ]
            }"

            export WEBKIT_DISABLE_COMPOSITING_MODE=1

            echo "Kaede dev shell ready! :3"
            echo "Hallo!!"
          '';
        };
      }
    );
}
