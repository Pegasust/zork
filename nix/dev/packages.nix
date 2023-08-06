{
  inputs,
  cell,
}: let
  pkgsLib = inputs.nixpkgs.lib;
  craneLib = inputs.boost.lib.craneLib;
  cleanSource = inputs.boost.lib.cleanRustSource';
  ws-cargo = builtins.fromTOML (builtins.readFile "${inputs.self}/Cargo.toml");
  basePathOf = orig_path: (baseNameOf (toString orig_path));
  crane-args = {
    src = cleanSource inputs.self (
      path: _:
        pkgsLib.any (
          suffix: pkgsLib.hasSuffix suffix (basePathOf path)
        ) [
          ".ungram"
        ]
    ) {};
    inherit (ws-cargo.workspace.metadata.nix) pname version;
    cargoExtraArgs = "--workspace";
  };
in {
  ws-deps = craneLib.buildDepsOnly crane-args;
  ws-all = craneLib.buildPackage (crane-args
    // {
      cargoArtifacts = cell.packages.ws-deps;
    });
}
