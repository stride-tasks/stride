{
  pkgs,
  nixos-lib,
  stride-cli,
  ...
}:
nixos-lib.runTest {
  hostPkgs = pkgs;
  name = "taskchampion-sync-server";

  nodes = {
    server = {config, ...}: {
      services.taskchampion-sync-server = {
        enable = true;
        openFirewall = true;
      };
    };
    stride_client = {config, ...}: {
      environment.systemPackages = [
        stride-cli
      ];
    };
    task_client = {config, ...}: {
      environment.systemPackages = [
        pkgs.taskwarrior3
      ];
    };
  };
  testScript = {nodes, ...}: let
    cfg = nodes.server.services.taskchampion-sync-server;
    port = builtins.toString cfg.port;

    # Generated with uuidgen
    uuid = "bf01376e-04a4-435a-9263-608567531af3";
    password = "nixos-test";

    user-dirs-file = pkgs.writeText "user-dirs.dirs" ''
      XDG_DOCUMENTS_DIR="/root/Documents"
    '';

    mkSyncConfig = path: quotes: let
      q =
        if quotes
        then "\""
        else "";
    in
      pkgs.writeShellScript "setup-config-file" ''
        set -xe

        mkdir --parents "$(dirname "${path}")"
        echo 'sync.server.origin=${q}http://server:${port}${q}' >> "${path}"
        echo 'sync.server.client_id=${q}${uuid}${q}' >> "${path}"
        echo 'sync.encryption_secret=${q}${password}${q}' >> "${path}"
      '';
  in
    /*
    python
    */
    ''
      start_all()

      server.wait_for_unit("taskchampion-sync-server.service")
      server.wait_for_open_port(${port})

      with subtest("Setup task syncing"):
          # See man task-sync(5)
          task_client.succeed("mkdir ~/.task")
          task_client.succeed("${mkSyncConfig "$HOME/.taskrc" false}")

          # The `TaskStorage` needs a document dir, which `stride` reads from the
          # `user-dirs.dirs` file (via the `dirs` crate)
          stride_client.succeed(
              "mkdir ~/.config",
              "cat ${user-dirs-file} > ~/.config/user-dirs.dirs",
              "mkdir ~/Documents"
          )
          stride_client.succeed("${mkSyncConfig "$HOME/.config/org.stridetasks.stride/config.toml" true}")

      with subtest("Can create tasks"):
          task_client.succeed("task add 'First task -- task_client'")
          stride_client.succeed("stride add 'First task -- stride_client'")

      with subtest("Can sync tasks"):
          task_client.succeed("task sync")
          stride_client.succeed("stride --sync task-champion'")
          task_client.succeed("task sync")

      with subtest("Have correct tasks"):
          count1 = task_client.succeed("task count")
          count2 = stride_client.succeed('stride search "" | wc -l')

          assert int(count1) == 2, f"We don't have exactly 2 tasks, but {count1}"
          assert count1 == count2, f"The clients don't have the same amount of tasks, stride_client: {count1}, task_client: {count2}"
    '';
}
