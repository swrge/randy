{
  description = "development shell for bot-worker";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";

    outputs = { self, nixpkgs }:
      let
        system = "x86_64-linux";
        pkgs = import nixpkgs { inherit system; config.allowUnfree = true; };
      in {
        devShells = {
          x86_64-linux = {
            default = pkgs.mkShellNoCC {
              packages = with pkgs.buildPackages; [
                # dev
                git openssh direnv typescript nodejs nodePackages.ts-node
                # networking
                curl
                # infra
                terraform ansible
                # cloud
                ngrok wrangler
              ];

              #shellHook = ''
              #  make
              #'';
            };
          };
        };
      };
}
