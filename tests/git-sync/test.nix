{
  pkgs,
  nixos-lib,
  stride-cli,
  ...
}: let
  sshKeys =
    import ./ssh_keys.nix {inherit pkgs;};
in
  nixos-lib.runTest {
    hostPkgs = pkgs;
    name = "git-sync";

    nodes = let
      clientConfig = {...}: {
        environment.systemPackages = [
          stride-cli
          pkgs.git
          pkgs.python312
        ];
      };
    in {
      server = {...}: {
        environment.systemPackages = [
          pkgs.git
        ];
        users = {
          users.git = {
            isNormalUser = true;
            openssh.authorizedKeys.keys = [
              sshKeys.admin.pub
            ];

            group = "git";
          };
          groups.git = {};
        };

        services.openssh = {
          enable = true;
          ports = [22];
          settings = {
            PasswordAuthentication = false;
            UseDns = true;
            X11Forwarding = false;
            PermitRootLogin = "prohibit-password";
            # Allow all MACs (otherwise `stride` fails to find a matching MAC to use when
            # communicating the sshd)
            Macs = null;
          };
        };
      };

      client_send = clientConfig;
      client_receive = clientConfig;
    };
    testScript = {nodes, ...}: let
      cfg = nodes.server;
      port = builtins.toString (builtins.elemAt cfg.services.openssh.ports 0);

      user-dirs-file = pkgs.writeText "user-dirs.dirs" ''
        XDG_DOCUMENTS_DIR="/root/Documents"
      '';

      strideTasksRepoPath = "/srv/git/stride-tasks.git";

      sshKeyUuid = "93d873d5-96c8-47df-80a4-d5edec397828";
      adjustConfig = pkgs.writeText "adjust_config.py" ''
        import json
        import sys

        with sys.stdin as f:
            d = json.load(f)
            d["repository"]["ssh_key_uuid"] = "${sshKeyUuid}"
            d["repository"]["origin"] = "git@server:${strideTasksRepoPath}"
            d["repository"]["encryption"]["key"] = "lfRpC9kVZmG3iw4Vaq1fTB5DETvmSndM3jLR6WAdnkM"
            print(json.dumps(d, sort_keys=True, indent=2))
      '';
      settingsPath = "/root/.local/share/org.stridetasks.stride";
      settingsJsonPath = "${settingsPath}/settings.json";
      settingsSshKeyPath = "${settingsPath}/.ssh/keys/${sshKeyUuid}";
    in
      /*
      python
      */
      ''
        start_all()

        with subtest("Setup ssh keys on clients"):
          for client in [client_send, client_receive]:
              client.succeed(
                  "mkdir -p ~root/.ssh",
                  "cp ${sshKeys.admin.priv} ~root/.ssh/id_ed25519",
                  "chmod 600 ~root/.ssh/id_ed25519",
              )

        with subtest("git server starts"):
          server.wait_for_unit("sshd.service")
          server.wait_for_open_port(${port})

        with subtest("Create server git repository"):
          server.succeed("${pkgs.writeShellScript "setup-git-repo" ''
          set -xe

          mkdir --parents ${strideTasksRepoPath}
          chown --recursive git:git ${strideTasksRepoPath}
          cd ${strideTasksRepoPath}

          # We set the `initial-branch` here, so that cloning this repository will result in
          # git checking out the `main` branch directly (which is what `stride` pushes),
          # instead of trying to checkout the default `master` (which will not exist).
          sudo --user=git git init --bare --initial-branch=main
        ''}")

        with subtest("Setup task syncing"):
          for client in [client_send, client_receive]:
              # The `TaskStorage` needs a document dir, which `stride` reads from the
              # `user-dirs.dirs` file (via the `dirs` crate)
              client.succeed(
                  "mkdir ~/.config",
                  "cat ${user-dirs-file} > ~/.config/user-dirs.dirs",
                  "mkdir ~/Documents"
              )

              # Create all the metadata files (with the exception of the repository)
              client.succeed('stride search ""')

              # This adds the required key attributes and such.
              client.succeed(
                  'echo "$(cat ${settingsJsonPath} | python3 ${adjustConfig})" > ${settingsJsonPath}'
              )
              # Add the ssh key
              client.succeed(
                "install -D ${sshKeys.admin.pubFile} ${settingsSshKeyPath}/key.pub",
                "install -D ${sshKeys.admin.priv} ${settingsSshKeyPath}/key",
              )
              # Add the hostKey
              client.succeed("ssh-keyscan -p ${port} -q server  > ${settingsPath}/.ssh/known_hosts")
              client.succeed("ssh-keyscan -p ${port} -q server  > ~root/.ssh/known_hosts")

          # Add the required git remote setup
          client_send.succeed("${pkgs.writeShellScript "git-remote-setup" ''
          set -xe

          # This also creates the `repository` directory, needed below
          stride add "Task 1 of client_send"

          cd ${settingsPath}/repository
          git remote add origin git@server:${strideTasksRepoPath}
          git push --set-upstream origin main
        ''}")
          client_receive.succeed("${pkgs.writeShellScript "git-remote-setup" ''
          set -xe

          git clone git@server:${strideTasksRepoPath} ${settingsPath}/repository
        ''}")

        with subtest("Can sync tasks"):
          client_send.succeed("stride sync")

        with subtest("Correct tasks were synced"):
          count_before = client_receive.succeed('stride search "" | wc -l')
          client_send.succeed('stride add "Second task" && stride sync')
          client_receive.succeed('stride sync')
          count_after = client_receive.succeed('stride search "" | wc -l')

          search_output = client_receive.succeed('stride search ""')
          expected_search_output = "    0: Task 1 of client_send\n    1: Second task\n"

          assert int(count_before) == 1, "Starting from a fresh clone should have 1 tasks"
          assert int(count_after) == 2, f"Did not receive 2 tasks from sync, but: '{count_after}'"
          assert search_output == expected_search_output, f"The tasks changed their message? Their new message is: '{search_output}', but we expected: '{expected_search_output}'"
      '';
  }
