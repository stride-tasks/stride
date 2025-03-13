{
  treefmt-nix,
  pkgs,
}:
treefmt-nix.lib.evalModule pkgs (
  {pkgs, ...}: {
    # Used to find the project root
    projectRootFile = "flake.nix";

    programs = {
      alejandra.enable = true;
      # rustfmt = {
      #   enable = true;
      #   edition = "2024";
      # };
      clang-format.enable = true;
      mdformat.enable = true;
      shfmt = {
        enable = true;
        indent_size = 2;
      };
      shellcheck.enable = true;
      prettier = {
        enable = true;
        settings = {
          arrowParens = "always";
          bracketSameLine = false;
          bracketSpacing = true;
          editorconfig = true;
          embeddedLanguageFormatting = "auto";
          endOfLine = "lf";
          # experimentalTernaries = false;
          htmlWhitespaceSensitivity = "css";
          insertPragma = false;
          jsxSingleQuote = true;
          printWidth = 80;
          proseWrap = "always";
          quoteProps = "consistent";
          requirePragma = false;
          semi = true;
          singleAttributePerLine = true;
          singleQuote = false;
          trailingComma = "all";
          useTabs = false;
          vueIndentScriptAndStyle = false;

          tabWidth = 2;
        };
      };
      stylua.enable = true;
      ruff = {
        enable = true;
        format = true;
      };
      taplo.enable = true;
    };

    settings = {
      formatter = {
        # rustfmt = {
        #   excludes = ["*/frb_generated.rs"];
        # };
        clang-format = {
          # options = ["--style" "Chromium"];
        };
        shfmt = {
          includes = ["*.bash"];
        };
      };
    };
  }
)
