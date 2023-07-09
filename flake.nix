{
  nixConfig = {
    extra-experimental-features = "nix-command flakes";
    accept-flake-config = true;
  };
  inputs = {
    boost.url = "git+https://git.pegasust.com/pegasust/nix-boost.git/bleed";
  };
  outputs = {

  };
}
