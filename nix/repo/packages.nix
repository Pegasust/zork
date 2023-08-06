{
  inputs,
  cell,
}: let
  inherit (inputs.cells.dev.packages) ws-all;
in {
  inherit ws-all;
  default = ws-all;
}
