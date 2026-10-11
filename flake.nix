{
  description = "tubular fetch";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem =
        {
          system,
          ...
        }:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        with pkgs.python3Packages;
        {
          packages.default = (
            buildPythonPackage {
              pname = "tfetch";
              version = "0.1.0";
              pyproject = true;

              src = ./.;

              build-system = [
                setuptools
              ];

              dependencies = [
                distro
                psutil
              ];
            }
          );

          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              python3
              psutil
              distro
            ];
          };
        };
    };
}
