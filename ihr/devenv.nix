{ pkgs, python-naoqi, ... }:

{
  # https://devenv.sh/basics/
  # env.GREET = "devenv";
  env.PYTHONPATH = "${python-naoqi.packages.x86_64-linux.naoqi}/lib/python2.7/site-packages";

  # https://devenv.sh/packages/
  packages = [
    pkgs.stdenv.cc.cc.lib
    pkgs.python27
    python-naoqi.packages.x86_64-linux.naoqi
  ];

  # https://devenv.sh/scripts/
  # scripts.hello.exec = "echo hello from $GREET";

  enterShell = ''
  '';

  # https://devenv.sh/languages/
  # languages.nix.enable = true;
  # languages.python.enable = true;
  # languages.python.version = "2.7";

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
