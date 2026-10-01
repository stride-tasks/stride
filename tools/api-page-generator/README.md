# API Page Generator

This crate builds a static HTML site for the JSON schema files under `api/`.

## Features

- Reads every schema file under `api/<kind>/`.
- Renders a clean, Tailwind-styled table of contents and schema reference page.
- Uses Tailwind via CDN; no npm or package install step is required.
- Keeps the schema `$id` values visible so a separate GitHub Pages CI step can rewrite or publish them correctly.

## Usage

From the repo root:

```bash
cargo run --bin api-page-generator
```

This writes the generated site to:

```text
tools/api-page-generator/output/index.html
```

The generated page can then be published by a separate GitHub Pages step.
