{
  inputs,
  cell,
}: {
  ungrammar_lsp = {
    type = "app";
    program = "${cell.packages.ws-all}/bin/ungrammar_lsp";
  };
}
