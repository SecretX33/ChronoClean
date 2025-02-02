# ChronoClean

ChronoClean is a fast, efficient, and safe file cleanup utility written in Rust that helps you automatically remove old files based on their timestamps while protecting important data.

## Features

- 🕒 Delete files based on their age (created, modified, or accessed time)
- 📁 Support for multiple target folders
- 🛡️ Ignore specific files and folders to protect important data
- 🌲 Configurable directory traversal depth
- 🗑️ Moves files to trash instead of permanent deletion
- 📝 Dry run mode to preview what would be deleted
- 🧹 Optional cleanup of empty folders
- 🔗 Optional symbolic link following

## Installation

### From Source
```bash
git clone https://github.com/SecretX33/ChronoClean.git
cd ChronoClean
cargo build --release
```

The compiled binary will be available in `target/release/`.

## Usage

```bash
chronoclean --delete-before <TIME> --target-folders <PATHS> [OPTIONS]
```

### Required Arguments

- `-d, --delete-before <TIME>`: Delete files older than this time (e.g., "30d", "24h", "1w", or combined like "3d 5h")
- `-t, --target-folders <PATHS>`: Comma-separated list of folders to clean up

### Optional Arguments

- `--file-date-types <TYPES>`: Specify which timestamps to check. You can use the full names (created, modified, accessed) or their first letters (c, m, a). [default: created,modified]
- `--ignored-paths <PATHS>`: Comma-separated list of files/folders to ignore
- `--min-depth <DEPTH>`: Minimum directory depth to search
- `--max-depth <DEPTH>`: Maximum directory depth to search
- `--delete-empty-folders`: Delete empty folders after file cleanup [default: false]
- `--follow-symbolic-links`: Follow symbolic links while traversing [default: false]
- `--dry-run`: Preview what would be deleted without actually deleting [default: false]

### Time Format

The time format for `--delete-before` supports various human-readable formats:
- `1s`, `1sec` - 1 second
- `2m`, `2min` - 2 minutes
- `3h`, `3hr` - 3 hours
- `4d`, `4days` - 4 days
- `5w`, `5week` - 5 weeks
- `6M`, `6month` - 6 months
- `7y`, `7year` - 7 years

You can also combine them: `1y6M` (1 year and 6 months), `2w3d` (2 weeks and 3 days), etc.

### Examples

#### Delete files older than 30 days in Downloads folder
```bash
chronoclean --delete-before 30d --target-folders "C:/Users/YourUser/Downloads"
```

#### Clean multiple folders, ignore specific paths
```bash
chronoclean --delete-before 1w --target-folders "Downloads,Documents" --ignored-paths "Documents/Important,Downloads/Keep"
```

#### Preview what would be deleted (dry run)
```bash
chronoclean --delete-before 24h --target-folders "Downloads" --dry-run
```

#### Delete old files and clean empty folders
```bash
chronoclean --delete-before 7d --target-folders "Downloads,Temp" --delete-empty-folders
```

#### Use specific timestamp types
```bash
chronoclean --delete-before 30d --target-folders "Downloads" --file-date-types "modified,accessed"
```

## Safety Features

1. Files are moved to the system trash instead of permanent deletion
2. Dry run mode to preview changes
3. Ignored paths to protect important data
4. Validation of all paths before operation
5. Detailed logging of all actions

## Building from Source

- Install [Rust](https://www.rust-lang.org/tools/install).
- Build the binary by executing this command, the compiled file will be in the `target/[debug|release]` folder.

```shell
# For development build
cargo build

# For release (optimized) build
cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the AGPL 3.0 License - see the LICENSE file for details. 