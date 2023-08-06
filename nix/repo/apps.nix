{
  inputs,
  cell,
}: {
  ungrammar_lsp = {
    type = "app";
    program = "${cell.ws-all}/bin/ungrammar_lsp";
  };
}
