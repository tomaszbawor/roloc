{ config, lib, ... }: with lib; let
  cfg = config.features.darwin.privateBrew;
in
{
  options.features.darwin.privateBrew.enable = mkEnableOption "Enable brew packages for private usage";

  config = mkIf cfg.enable {
    homebrew = {
      brews = [
      ];
      casks = [
        "calibre"
      ];
    };
  };
}
