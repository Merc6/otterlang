{
  description = "otterlang flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        hasInfix = pkgs.lib.strings.hasInfix;
      in
      {
        devShells.default =
          with pkgs;
          let
            rust_toolchain = (rust-bin.nightly.latest.default.override {
              extensions = [
                "rust-src"
                "rustc-codegen-cranelift-preview"
              ];
            });
          in
          mkShell {
            buildInputs = [
              rust_toolchain
            ]
            ++ lib.optionals (hasInfix "linux" system) [
              mold
            ];

            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

            RUSTFLAGS =
              "-Zshare-generics=y" + lib.optionalString (hasInfix "linux" system) " -Clink-arg=-fuse-ld=mold";

            CARGO_PROFILE_DEV_CODEGEN_BACKEND = (if hasInfix "linux" system then "cranelift" else "llvm");
            CARGO_NET_GIT_FETCH_WITH_CLI = "true";
          };
      }
    );
}
