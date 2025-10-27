{
  description = "IC10emu";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
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

        dioxusCargoDeps =
          pkgs.rustPlatform.importCargoLock { lockFile = ./Dioxus.lock; };

        dioxus-cli = pkgs.dioxus-cli.overrideAttrs (_: {
          postPatch = ''
            rm Cargo.lock
            cp ${./Dioxus.lock} Cargo.lock
          '';
          cargoDeps = dioxusCargoDeps;
          version = "0.7.0-rc.3";
          src = pkgs.fetchCrate {
            pname = "dioxus-cli";
            version = "0.7.0-rc.3";
            hash = "sha256-wyEN/WH4OEHrZk9zdOzBkRYDHVXoHUaeGd2Heegrx/I=";
          };
          patches = [ ];
          cargoHash = "";

          cargoBuildFeatures = [ "no-downloads" ];
          cargoCheckFeatures = [ "no-downloads" ];

          doCheck = false;

        });

        cargoLock = builtins.fromTOML (builtins.readFile ./Cargo.lock);

        wasmBindgen = pkgs.lib.findFirst (pkg: pkg.name == "wasm-bindgen")
          (throw "Could not find wasm-bindgen package") cargoLock.package;

        wasm-bindgen-cli = pkgs.buildWasmBindgenCli rec {
          src = pkgs.fetchCrate {
            pname = "wasm-bindgen-cli";
            version = wasmBindgen.version;
            hash = "sha256-9kW+a7IreBcZ3dlUdsXjTKnclVW1C1TocYfY8gUgewE=";
          };

          cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
            inherit src;
            inherit (src) pname version;
            hash = "sha256-V0AV5jkve37a5B/UvJ9B3kwOW72vWblST8Zxs8oDctE=";
          };
        };

      in {
        devShells.default = pkgs.mkShell {
          name = "ic10emu";

          buildInputs = [
            toolchain
            pkgs.rust-analyzer-unwrapped
            llvm.bintools
            llvm.clang-unwrapped
            llvm.clang-tools
            llvm.libclang
            pkgs.cargo
            pkgs.cargo-watch
            pkgs.cargo-dist
            pkgs.cargo-edit
            pkgs.cargo-insta
            pkgs.turso-cli
            pkgs.nodejs
            pkgs.pnpm
            pkgs.python3
            pkgs.wasm-pack
            pkgs.wabt
            pkgs.tree-sitter

            pkgs.eza
            pkgs.fd

            wasm-bindgen-cli
            dioxus-cli
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
