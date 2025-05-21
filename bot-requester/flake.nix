{
  description = "development shell for bot-requester";

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
                # misc
                git openssh direnv
                # networking
                curl
                # infra
                terraform ansible_2_18
                # cloud
                google-cloud-sdk
              ];

              #shellHook = ''
              #  make
              #'';
            };
          };
        };
      };
}
