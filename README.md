# tloc (Toad Lines of Code) 🐸
tloc is a lightweight CLI tool built to quickly get a markdown list of all the solidity files within a directory, their SLOC and the total SLOC of the codebase.

## Motivation
The output of this tool is something I have been doing manually at the start of every audit engagement. I've always preferred having the list of files and their associated lines of code in a markdown file as it allows me to easily check off files after review them and add notes along the way.

This tool greatly speeds that up, and it's written in Rust so... blazingly fast™️.

## Installation
```bash
git clone https://github.com/mccoady/tloc
cd tloc
cargo install --path .
```

## Usage
```bash
tloc --path PATH_TO_DIRECTORY --out PATH_TO_WRITE_OUTPUT
```

**Options**
```bash
# Perform recursive search through any nested directorys within path 
tloc --path PATH --out OUT --recurse
```

**Example Output**
```markdown
# Scope
- `FileOne.sol` (203)
- `FileTwo.sol` (603)
TOTAL: 806
```

## Contributing
Contributions are welcome!
