{
  inputs,
  cell,
}: let
  inherit (inputs.cells.dev.packages) ws-all;
in {
  inherit ws-all;
  default = ws-all;
  ungrammar_lsp = inputs.nixpkgs.writeShellApplication {
    name = "ungrammar_lsp";
    text = ''
      UNGRAMMAR_LSP_CONF="$HOME/.config/ungrammar_lsp.toml" \
      RUST_LOG=TRACE \
      ${cell.packages.ws-all}/bin/ungrammar_lsp
    '';
  };
}
