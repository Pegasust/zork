{inputs, cell}: let
  craneLib = inputs.boost.lib.craneLib;
  cleanSource = inputs.boost.lib.cleanRustSource;
  ws-cargo = (builtins.fromTOML (builtins.readFile "${inputs.self}/Cargo.toml"));
  crane-args = {
    src = cleanSource inputs.self;
    inherit (ws-cargo.workspace.metadata.nix) pname version;
    cargoExtraArgs = "--workspace";
  };
in {
  ws-deps = craneLib.buildDepsOnly crane-args;
  ws-all = craneLib.buildPackage (crane-args // {
    cargoArtifacts = cell.packages.ws-deps;
  });
}
