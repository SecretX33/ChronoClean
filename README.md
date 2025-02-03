# ChronoClean

[![CI](https://github.com/SecretX33/ChronoClean/actions/workflows/build-and-release.yml/badge.svg)](https://github.com/SecretX33/ChronoClean/actions/workflows/build-and-release.yml)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/SecretX33/ChronoClean)](https://github.com/SecretX33/ChronoClean/releases/latest)
[![GitHub License](https://img.shields.io/github/license/SecretX33/ChronoClean)](https://github.com/SecretX33/ChronoClean/blob/main/LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)

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

## Download

ChronoClean is available for Windows, Linux, and MacOS. 

Get the latest version [here](https://github.com/SecretX33/ChronoClean/releases/latest). Want an older version? Check all releases [here](https://github.com/SecretX33/ChronoClean/releases).

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
- `--min-depth <DEPTH>`: Minimum directory depth to search [default: 0]
- `--max-depth <DEPTH>`: Maximum directory depth to search [default: infinite]
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
chronoclean --delete-before 30d --target-folders "C:/Users/User/Downloads"
```

#### Clean multiple folders, ignore specific paths
```bash
chronoclean --delete-before 1w --target-folders "C:/Users/User/Downloads","C:/Users/User/Documents" --ignored-paths "C:/Users/User/Downloads/Keep","C:/Users/User/Documents/Important"
```

#### Preview what would be deleted (dry run)
```bash
chronoclean --delete-before 24h --target-folders "C:/Users/User/Downloads" --dry-run
```

#### Delete old files and clean empty folders
```bash
chronoclean --delete-before 7d --target-folders "C:/Users/User/Downloads","C:/Users/User/Temp" --delete-empty-folders
```

#### Use specific timestamp types
```bash
chronoclean --delete-before 30d --target-folders "C:/Users/User/Downloads" --file-date-types "modified,accessed"
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

Contributions are welcome! Please feel free to submit a Pull Request with your changes, or open an Issue to request new features.

## License

This project is licensed under the AGPL 3.0 license. See the [LICENSE](LICENSE) file for details.