{
    description = "hpr";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        crane = { 
            url = "github:ipetkov/crane"; 
            inputs.nixpkgs.follows = "nixpkgs"; 
        };
        fenix = { 
            url = "github:nix-community/fenix"; 
            inputs.nixpkgs.follows = "nixpkgs"; 
            inputs.rust-analyzer-src.follows = ""; 
        };
    };

    outputs = inputs @ { self, nixpkgs, crane, fenix, ... }:
    let 
        forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" ];
        nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in {
        packages = forAllSystems (system: {
            hpr = let
                pkgs = nixpkgsFor.${system};
                craneLib = crane.lib.${system};
            in craneLib.buildPackage {
                src = craneLib.cleanCargoSource (craneLib.path ./.);
                strictDeps = true;

                buildInputs = [];
            };

            default = self.packages.${system}.hpr;
        });

        checks = forAllSystems (system: { 
            inherit (self.packages.${system}.hpr);

            hpr-fmt = let craneLib = crane.lib.${system}; in craneLib.cargoFmt { 
                src = craneLib.cleanCargoSource (craneLib.path ./.);
            };
        });

        apps = forAllSystems (system: {
            default = {
                type = "app";
                program = "${self.packages.${system}.hpr}/bin/hpr"; 
            };
        });

        devShells = forAllSystems (system: {
            default = let 
                pkgs = nixpkgsFor.${system};
                #db_host = "";
                db_name = "hpr";
                db_user = "hpr";
                db_password = "test";
                db_path = "temp/hpr";
            in pkgs.mkShell {
                buildInputs = [ 
                    fenix.packages.${system}.complete.toolchain 
                    pkgs.cargo-watch
                    pkgs.mold-wrapped
                    pkgs.cmake
                    pkgs.opencascade-occt
                ];
            };
            hpr = crane.lib.${system}.devShell {
                checks = self.checks.${system};

                packages = [];
            };
        });
    };

}
