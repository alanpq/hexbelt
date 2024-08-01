{
  inputs = {
    # nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    rust-overlay.url = "github:oxalica/rust-overlay";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, systems, nixpkgs, treefmt-nix, ... }@inputs:
    let
      eachSystem = f: nixpkgs.lib.genAttrs (import systems) (
        system: f (
          import nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          }
        )
      );
      rustToolchain = eachSystem (pkgs: (pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" ];
        targets = [ "wasm32-unknown-unknown" ];
      }));
      treefmtEval = eachSystem (pkgs: treefmt-nix.lib.evalModule pkgs ./treefmt.nix);
    in
    {
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            clang
            # Use mold when we are runnning in Linux
            (lib.optionals stdenv.isLinux mold)
          ];
          buildInputs = with pkgs; [
            nodejs

            # You can set the major version of Node.js to a specific one instead
            # of the default version
            # pkgs.nodejs-22_x

            # It is possible to use bun instead of node.
            # pkgs.bun

            # Optionally, you can add yarn or pnpm for package management for node.
            # pkgs.nodePackages.pnpm
            yarn

            nodePackages.typescript
            nodePackages.typescript-language-server
            svelte-language-server

            rustToolchain.${pkgs.system}
            rust-analyzer-unwrapped
            cargo
            wasm-pack
            wabt
          ];
          RUST_SRC_PATH = "${rustToolchain.${pkgs.system}}/lib/rustlib/src/rust/library";
        };
      });
      formatter = eachSystem (pkgs: treefmtEval.${pkgs.system}.config.build.wrapper);

      checks = eachSystem (pkgs: {
        formatting = treefmtEval.${pkgs.system}.config.build.check self;
      });
    };
}
