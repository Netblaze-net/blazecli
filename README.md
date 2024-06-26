---

# BlazeCLI

## Description

BlazeCLI is a command-line interface tool that allows you to generate new projects based on predefined templates. It's built using Rust and leverages libraries like `clap` for command-line argument parsing, `handlebars` for templating, and `rust-embed` for embedding static assets.

## Features

- Template-based project generation
- Embedded template files for portability
- Dynamic variable replacement in templates
- Easy to extend with new templates

## Installation

To install BlazeCLI, you'll need to have Rust and Cargo installed. You can then clone the repository and build the project:

```bash
git clone https://github.com/Netblaze-net/blazecli.git
cd blazecli
cargo build --release
```

## Usage

### Generate a New Project

To generate a new project based on a predefined template:

```bash
blazecli generate <template_name>
```

The CLI will prompt you for any variables required by the template, and then create the new project in a directory named after your project.

### List Available Templates

To list all available templates:

```bash
blazecli generate
```

This will display the names of all embedded templates you can use.

---
