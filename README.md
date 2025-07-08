<div align="center">
<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 align="center">🚧 Maintenance page component for Edgee</h1>

[![Coverage Status](https://coveralls.io/repos/github/edgee-cloud/maintenance-page-component/badge.svg)](https://coveralls.io/github/edgee-cloud/maintenance-page-component)
[![GitHub issues](https://img.shields.io/github/issues/edgee-cloud/maintenance-page-component.svg)](https://github.com/edgee-cloud/maintenance-page-component/issues)
[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.cloud/edgee/maintenance-page)


This component provides a simple maintenance page on [Edgee](https://www.edgee.cloud),
served directly at the edge. It's meant to be mapped to the root of your website, but you can
also map it to specific sub-paths.


## Quick Start

1. Download the latest component version from our [releases page](../../releases)
2. Place the `maintenance.wasm` file in your server (e.g., `/var/edgee/components`)
3. Add the following configuration to your `edgee.toml`:

```toml
[[components.edge_functions]]
id = "maintenance"
file = "/var/edgee/components/maintenance.wasm"
settings.edgee_path = "/path" # exact match
settings.edgee_path_prefix = "/prefix" # will match /prefix/anything
```
Note that either `edgee_path` or `edgee_path_prefix` must be set, but not both.

## Development

### Building from Source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)

Build command:
```bash
edgee component build
```

Test command (with local HTTP emulator):
```bash
edgee component test
```

Test coverage command:
```bash
make test.coverage[.html]
```

### Contributing
Interested in contributing? Read our [contribution guidelines](./CONTRIBUTING.md)

### Security
Report security vulnerabilities to [security@edgee.cloud](mailto:security@edgee.cloud)
