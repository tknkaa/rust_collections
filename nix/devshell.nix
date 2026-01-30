{ pkgs }:
pkgs.mkShell {
  # Add build dependencies
  packages = with pkgs; [
    apacheHttpd
  ];

  # Add environment variables
  env = { };

  # Load custom bash code
  shellHook = ''

  '';
}
