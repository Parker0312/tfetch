{
  description = "";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;

      pname = "tfetch";
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = self.packages.${system}.tfetch;
          ${pname} = pkgs.rustPlatform.buildRustPackage {
            name = pname;
            src = ./.;
            cargoHash = "sha256-NaUlZNEVy83OhWTYqrc/UQhH5G2duwgIBzZYOq78SWA=";
          };
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
              buildInputs = [ pkgs.openssl ];
              nativeBuildInputs = [ pkgs.pkg-config ];
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];
          };
        }
      );
    };
}
