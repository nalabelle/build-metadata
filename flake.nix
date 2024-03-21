{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    devshell.url = "github:numtide/devshell";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = inputs @ {nixpkgs, devshell, rust-overlay, ...}: let
    overlays = [ (import rust-overlay) devshell.overlays.default ];
    forAllSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ] (system:
        function (import nixpkgs {
          inherit system overlays;
          # config.allowUnfree = true;
        }));
  in {
    packages = forAllSystems (pkgs: {
      default = pkgs.devshell.mkShell {
        imports = [ (pkgs.devshell.importTOML ./devshell.toml) ];
        devshell.packages = [ pkgs.pkg-config pkgs.openssl ];
      };
    });
    nixosConfigurations.nixos = nixpkgs.lib.nixosSystem {};
    overlays.default = final: prev: {};
  };
}
