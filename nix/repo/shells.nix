{inputs, cell}: let 
  inherit (inputs.std) std lib;
  pkgs = inputs.nixpkgs;
in {
  rust-dev = lib.dev.mkShell {
    name = "rust-dev";
    imports = [
      inputs.boost.devshellProfiles.rust-klepto
    ];
    # env = [
    #   {
    #     name="PATH";
    #     prefix="${pkgs.xcbuild}/bin";
    #   }
    #   {
    #     name="PATH";
    #     prefix="${pkgs.xcbuild.toolchain}/bin";
    #   }
    # ];
  };
}
