{
  description = "IC10emu";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = {
      self,
      nixpkgs,
     ...
    }:
    let
      inherit (nixpkgs) lib;
      systems = lib.systems.flakeExposed;

      forAllSystems = lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system: nixpkgs.legacyPackages.${system});
    in {
      devShells = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
          llvm = pkgs.llvmPackages_20;

        in {
          default = pkgs.mkShell {
            name = "ic10emu";

            buildInputs = with pkgs; [
              llvm.bintools
              llvm.clang-tools
              cargo
              cargo-watch
              cargo-dist
              cargo-edit
              cargo-insta
              rustc
              rust-analyzer
              rustfmt
              clippy
              wasm-bindgen-cli
              dioxus-cli
              turso-cli
              nodejs
              pnpm
            ];

            # # Add glibc, clang, glib, and other headers to bindgen search path
            BINDGEN_EXTRA_CLANG_ARGS =
              # Includes normal include path
              (builtins.map (a: ''-I"${a}/include"'') [
                # add dev libraries here (e.g. pkgs.libvmi.dev)
                pkgs.glibc.dev
              ])
              # Includes with special directory paths
              ++ [
                ''
                  -I"${llvm.libclang.lib}/lib/clang/${llvm.libclang.version}/include"''
                ''-I"${pkgs.glib.dev}/include/glib-2.0"''
                "-I${pkgs.glib.out}/lib/glib-2.0/include/"
              ];

            CPLUS_INCLUDE_PATH = (builtins.map (a: ''"${a}/include"'') [
              # add dev libraries here (e.g. pkgs.libvmi.dev)
              pkgs.glibc.dev
            ])
            # Includes with special directory paths
              ++ [
                ''
                  "${llvm.libclang.lib}/lib/clang/${llvm.libclang.version}/include"''
                ''"${pkgs.glib.dev}/include/glib-2.0"''
                "${pkgs.glib.out}/lib/glib-2.0/include/"
              ];

            shellHook = "";
          };
        });
    };
}
