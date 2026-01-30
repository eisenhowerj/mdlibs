# mdlibs 💀🔥

no cap fr fr this is bussin - markdown library CLI tool coded in rust (rust mentioned??? 🦀)

## Overview 👀

yo mdlibs slaps different ngl. its giving CLI tool energy for managing ur markdown docs. like literally just vibing with libraries, listing docs, updating metadata and searching thru files. its lowkey highkey essential fr.

## Installation 📦

### From Source (if u can even code lmao)

```bash
git clone https://github.com/eisenhowerj/mdlibs.git
cd mdlibs
cargo build --release
```

binary gonna spawn at `target/release/mdlibs` (real)

## Usage 🎮

### Initialize a new markdown library

```bash
mdlibs init [path]
```

spawns a new markdown library wherever u want (current dir by default bc ur lazy probably)

### List documents

```bash
mdlibs list [--filter <filter>]
```

lists all ur markdown docs. add filter if ur organized (unlikely)

### Update documents

```bash
mdlibs update <document> [--title <title>]
```

changes metadata or content. touch grass after tho.

### Search documents

```bash
mdlibs search <query> [--title-only]
```

searches thru ur docs. `--title-only` flag if u got that adhd (relatable)

## Examples 📝

```bash
# init library (lfg)
mdlibs init

# init in specific dir (so organized bestie)
mdlibs init ~/my-notes

# list everything (overstimulation incoming)
mdlibs list

# filter that shiiii
mdlibs list --filter "tutorial"

# change title (rebrand era fr)
mdlibs update doc1.md --title "New Title"

# search szn
mdlibs search "rust programming"

# title search only (minimalist queen)
mdlibs search "tutorial" --title-only
```

## Development 🛠️

### Building

```bash
cargo build
```

ez clap

### Running tests

```bash
cargo test
```

pls dont fail pls dont fail

### Running the CLI

```bash
cargo run -- <command> [args]
```

## License ⚖️

MIT License check the [LICENSE](LICENSE) file if u actually care (u dont)

## Contributing 🤝

contributions are bussin! read [CONTRIBUTING.md](CONTRIBUTING.md) for the lore and how to submit PRs (pull requests not public relations smh)

---

*if u read this whole thing u definitely got that neurodivergent hyperfixation energy. respect. now go touch grass bestie* 🌱✨
