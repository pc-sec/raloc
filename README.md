# raloc (Rust Actual Lines of Code)
tloc is a lightweight CLI tool built in with Rust to quickly get a markdown list of all the rust files within a directory, their SLOC and the total SLOC of the codebase whilst not counting lines from inline unit tests.

## Installation
```bash
git clone https://github.com/pc-sec/raloc
cd raloc
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
- `file_one.rs` (203)
- `file_two.rs` (603)
TOTAL: 806
```

