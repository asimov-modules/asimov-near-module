# ASIMOV NEAR Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-near-module.svg)](https://crates.io/crates/asimov-near-module)
[![Package on PyPI](https://img.shields.io/pypi/v/asimov-near-module.svg)](https://pypi.org/project/asimov-near-module)
[![Package on RubyGems](https://img.shields.io/gem/v/asimov-near-module.svg)](https://rubygems.org/gems/asimov-near-module)
[![Package on NPM](https://img.shields.io/npm/v/asimov-near-module.svg)](https://npmjs.com/package/asimov-near-module)

[ASIMOV] module for data import from the [NEAR Protocol] blockchain network.

## ✨ Features

- Imports structured data from [NEAR Protocol] blocks and transactions.
- Supports both the [mainnet] and [testnet] environments.
- Collects the raw JSON data via the edge-cached [neardata.xyz] API.
- Constructs a semantic knowledge graph based on the [KNOW] ontology.
- Supports plain JSON output as well as [RDF] output in the form of [JSON-LD].
- Distributed as a standalone static binary with zero runtime dependencies.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation from PyPI

```bash
pip install -U asimov-near-module
```

### Installation from RubyGems

```bash
gem install asimov-near-module
```

### Installation from NPM

```bash
npm install -g asimov-near-module
```

### Installation from Source Code

```bash
cargo install asimov-near-module
```

## 👉 Examples

### Fetching Testnet Blocks

```bash
asimov-near-fetcher near://testnet/100000000
```

### Fetching Mainnet Blocks

```bash
asimov-near-fetcher near://mainnet/100000000
```

## 📚 Reference

### Installed Binaries

- `asimov-near-fetcher`: collects JSON data from the [neardata.xyz] API
- `asimov-near-importer`: collects and transforms JSON into JSON-LD (TBD)

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-near-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-near-module&text=asimov-near-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-near-module&title=asimov-near-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-near-module&t=asimov-near-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-near-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-near-module)

[ASIMOV]: https://github.com/asimov-platform
[JSON-LD]: https://json-ld.org
[KNOW]: https://github.com/know-ontology
[NEAR Protocol]: https://near.org
[NPM]: https:/npmjs.org
[Python]: https://python.org
[RDF]: https://www.w3.org/TR/rdf12-concepts/
[Ruby]: https://ruby-lang.org
[Rust]: https://rust-lang.org
[mainnet]: https://docs.near.org/protocol/network/networks#mainnet
[neardata.xyz]: https://neardata.xyz
[testnet]: https://docs.near.org/protocol/network/networks#testnet
