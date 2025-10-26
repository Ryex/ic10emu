{
  description = "IC10emu";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        llvm = pkgs.llvmPackages;
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        fixed_include_path = (builtins.map (a: ''"${a}/include"'') [
          # add dev libraries here (e.g. pkgs.libvmi.dev)
          pkgs.glibc.dev
        ])
        # Includes with special directory paths
          ++ [
            ''
              "${llvm.libclang.lib}/lib/clang/${
                pkgs.lib.versions.major llvm.libclang.version
              }/include"''
            ''"${pkgs.glib.dev}/include/glib-2.0"''
            "${pkgs.glib.out}/lib/glib-2.0/include/"
          ];

      in {
        devShells.default = with pkgs;
          mkShell {
            name = "ic10emu";

            buildInputs = with pkgs; [
              toolchain
              pkgs.rust-analyzer-unwrapped
              llvm.bintools
              llvm.clang-unwrapped
              llvm.clang-tools
              llvm.libclang
              cargo
              cargo-watch
              cargo-dist
              cargo-edit
              cargo-insta
              wasm-bindgen-cli
              dioxus-cli
              turso-cli
              nodejs
              pnpm
              python3
              wasm-pack
              wabt
              tree-sitter

              eza
              fd
            ];

            RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
            CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";

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
                  -I"${llvm.libclang.lib}/lib/clang/${
                    pkgs.lib.versions.major llvm.libclang.version
                  }/include"''
                ''-I"${pkgs.glib.dev}/include/glib-2.0"''
                "-I${pkgs.glib.out}/lib/glib-2.0/include/"
              ];

            CPLUS_INCLUDE_PATH = fixed_include_path;
            C_INCLUDE_PATH = fixed_include_path;

            shellHook = ''
              alias ls=eza
              alias find=fd
              # export CC="clang"
              echo "CC is set to: $CC"
            '';
          };
      });
}
