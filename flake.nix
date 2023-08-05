{
  nixConfig = {
    extra-experimental-features = "nix-command flakes";
    accept-flake-config = true;
  };
  inputs = {
    # nixpkgs.url = "github:pegasust/nixpkgs/staging-xcbuild-drv";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    boost.url = "git+https://git.pegasust.com/pegasust/nix-boost.git?ref=bleed";
  };
  outputs = inps@{self, boost, nixpkgs, ...}: 
    let inherit (boost.inputs) std;
  in std.growOn {
    inputs = inps // {std = std;};
    cellsFrom = ./nix;
    cellBlocks = 
    let inherit (std.blockTypes) devshells functions installables; 
    in [
      (functions "shellProfiles")
      (devshells "shells")
      (devshells "userShells")
      (installables "packages")
    ];
  } {
    packages = std.harvest self [["repo" "packages"]];
    devShells = std.harvest self [["repo" "shells"] ["repo" "userShells"]];
    devshellProfiles = std.harvest self [["repo" "shellProfiles"]];
    inherit nixpkgs;
  };
}
