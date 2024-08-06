{
  description = "ralsay in rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    systems = [ "x86_64-linux" "aarch64-linux" ];
    forEachSystems = nixpkgs.lib.genAttrs systems;
    pkgsForEach = nixpkgs.legacyPackages;
  in
  {
    packages = forEachSystems (system: {
      default = pkgsForEach.${system}.callPackage ./nix/package.nix { };
    });

    devShells = forEachSystems (system: {
      default = pkgsForEach.${system}.callPackage ./nix/shell.nix { };
    });
  };
}
