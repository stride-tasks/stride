{pkgs}: {
  admin = rec {
    priv = pkgs.writeText "id_ed25519" ''
      -----BEGIN OPENSSH PRIVATE KEY-----
      b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
      QyNTUxOQAAACDu7qxYQAPdAU6RrhB3llk2N1v4PTwcVzcX1oX265uC3gAAAJBJiYxDSYmM
      QwAAAAtzc2gtZWQyNTUxOQAAACDu7qxYQAPdAU6RrhB3llk2N1v4PTwcVzcX1oX265uC3g
      AAAEDE1W6vMwSEUcF1r7Hyypm/+sCOoDmKZgPxi3WOa1mD2u7urFhAA90BTpGuEHeWWTY3
      W/g9PBxXNxfWhfbrm4LeAAAACGJmb0BtaW5pAQIDBAU=
      -----END OPENSSH PRIVATE KEY-----
    '';

    pub = ''
      ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIO7urFhAA90BTpGuEHeWWTY3W/g9PBxXNxfWhfbrm4Le root@client
    '';
    pubFile = pkgs.writeText "id_ed25519.pub" pub;
  };
}
