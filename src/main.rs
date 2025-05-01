use clap::Parser;
use color_eyre::eyre;
use color_eyre::eyre::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use walkdir::{DirEntry, WalkDir};

mod log_macros;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, required = true, value_name = "TIME", value_parser = humantime::parse_duration, help = "Delete files older than this time")]
    delete_before: Duration,

    #[arg(short, long, required = true, value_name = "PATH", value_delimiter = ',', help = "The folder to delete files from")]
    target_folders: Vec<PathBuf>,

    #[arg(long, default_value = "created,modified", value_delimiter = ',', value_parser = file_date_type_parser, help = "Print debug information")]
    file_date_types: Vec<FileDateType>,

    #[arg(long, value_name = "PATHS", value_delimiter = ',', help = "Add a file or folder as ignored, files ignored and files inside folders ignored will not be deleted")]
    ignored_paths: Option<Vec<PathBuf>>,

    #[arg(long, value_name = "DEPTH", help = "Minimum depth to search for files to delete")]
    min_depth: Option<usize>,

    #[arg(long, value_name = "DEPTH", help = "Maximum depth to search for files to delete")]
    max_depth: Option<usize>,

    #[arg(long, default_value = "false", help = "Delete empty folders from all target folders after deleting files (default: false)")]
    delete_empty_folders: bool,

    #[arg(long, default_value = "false", help = "Follow symbolic links (default: false)")]
    follow_symbolic_links: bool,

    #[arg(long, default_value = "false", help = "Don't delete the files, just say which files would be deleted (default: false)")]
    dry_run: bool,
}

#[derive(Parser, Debug, PartialEq, Clone, Copy)]
#[clap(about = "The type of date to use for comparison", rename_all = "snake_case")]
enum FileDateType {
    Created,
    Modified,
    Accessed,
}

fn file_date_type_parser(value: &str) -> Result<FileDateType, String> {
    let trimmed_value = value.trim();
    match trimmed_value.to_ascii_lowercase().as_str() {
        "c" | "created" => Ok(FileDateType::Created),
        "m" | "modified" => Ok(FileDateType::Modified),
        "a" | "accessed" => Ok(FileDateType::Accessed),
        _ => Err(format!("Unsupported file date type: {}. Please use one of the following: {}", trimmed_value, ["created (c)", "modified (m)", "accessed (a)"].join(", "))),
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    validate_arguments(&cli)?;
    print_arguments(&cli);

    let files_to_delete = get_files_to_delete(&cli)?;
    delete_files(&cli, &files_to_delete);
    delete_empty_folders_in_target_folders(&cli)?;
 
    Ok(())
}

fn validate_arguments(cli: &Cli) -> Result<()> {
    for target_folder in cli.target_folders.iter() {
        if !target_folder.exists() {
            return Err(eyre::eyre!(format!("The target folder does not exist: {}", target_folder.display())));
        }
    }

    if let Some(ignored_paths) = &cli.ignored_paths {
        for path in ignored_paths {
            if !path.exists() {
                return Err(eyre::eyre!(format!("The ignored path does not exist: {}", path.display())));
            }
        }
    }
    
    if let (Some(min_depth), Some(max_depth)) = (cli.min_depth, cli.max_depth) {
        if min_depth > max_depth {
            return Err(eyre::eyre!("The minimum depth must be less than or equal to the maximum depth"));
        }
    }
    
    Ok(())
}

fn print_arguments(cli: &Cli) {
    log!("These are the arguments you provided:");
    log!("Delete before: {}", humantime::format_duration(cli.delete_before));
    log!("Target folders: {:?}", cli.target_folders.iter().map(|p| p.display()).collect::<Vec<_>>());
    log!("Finding files to delete by their: {:?}", cli.file_date_types);
    if let Some(ignored_paths) = &cli.ignored_paths {
        log!("Ignored paths: {:?}", ignored_paths.iter().map(|p| p.display()).collect::<Vec<_>>());
    }
    if let Some(min_depth) = cli.min_depth {
        log!("Min depth: {}", min_depth);
    }
    if let Some(max_depth) = cli.max_depth {
        log!("Max depth: {}", max_depth);
    }
    log!("Delete empty folders: {}", cli.delete_empty_folders);
    log!("Follow symbolic links: {}", cli.follow_symbolic_links);
    log!("Dry run: {}", cli.dry_run);
    log!("");
}

fn get_files_to_delete(cli: &Cli) -> Result<Vec<PathBuf>> {
    let mut files_to_delete = Vec::new();

    let now = std::time::SystemTime::now();
    let cutoff = now - cli.delete_before;

    log!("Finding files to delete in target folder...");

    for entry in walk_target_folders(&cli) {
        if entry.is_err() {
            log!("Failed to read entry: {:?}", entry.err().unwrap());
            continue;
        }

        let entry = entry?;
        let path = entry.path();

        let is_inside_ignored_folder = cli.ignored_paths.as_ref()
            .map_or(false, |ignored_paths| ignored_paths.iter().any(|ignored_path| path.starts_with(ignored_path)));
        if is_inside_ignored_folder {
            continue;
        }

        if path.is_file() {
            let metadata = path.metadata()?;

            let created = metadata.created()?;
            let modified = metadata.modified()?;
            let accessed = metadata.accessed()?;

            let file_time = cli.file_date_types.iter()
                .map(|t| match t {
                    FileDateType::Created => created,
                    FileDateType::Modified => modified,
                    FileDateType::Accessed => accessed,
                }).max()
                .expect("At least one file date type must is provided");
            
            if file_time <= cutoff {
                files_to_delete.push(path.to_path_buf());
            }
        }
    }
    log!("Found {} files to delete", files_to_delete.len());

    Ok(files_to_delete)
}

fn delete_files(cli: &Cli, files_to_delete: &[PathBuf]) {
    log!("Deleting files...");

    let max = files_to_delete.len();

    for (index, path) in files_to_delete.iter().enumerate() {
        if cli.dry_run {
            log!("{}/{}. Would delete file: {}", index + 1, max, path.display());
        } else {
            log!("{}/{}. Deleting file: {}", index + 1, max, path.display());
            if let Err(e) = trash::delete(path) {
                log!("Failed to move file '{}' to trash: {:?}", path.display(), e);
            }
        }
    }

    log!("Finish deleting files");
}

fn walk_target_folders(cli: &Cli) -> impl Iterator<Item = Result<DirEntry>> + use<'_> {
    fn walk_folder(
        folder: &Path,
        cli: &Cli,
    ) -> Option<impl Iterator<Item = Result<DirEntry>>> {
        if !folder.is_dir() {
            return None;
        }
        let mut walk = WalkDir::new(&folder).follow_links(cli.follow_symbolic_links);

        if let Some(min_depth) = cli.min_depth {
            walk = walk.min_depth(min_depth);
        }
        if let Some(max_depth) = cli.max_depth {
            walk = walk.max_depth(max_depth);
        }

        Some(walk.into_iter().map(|e| e.map_err(|e| eyre::eyre!(e))))
    }
    
    cli.target_folders.iter()
        .flat_map(|e| walk_folder(e, cli).into_iter().flatten())
}

fn delete_empty_folders_in_target_folders(cli: &Cli) -> Result<()> {
    if !cli.delete_empty_folders {
        return Ok(());
    }
    
    let counter = AtomicU32::new(0);
    log!("\nDeleting empty folders...");
    for target_folder in cli.target_folders.iter() {
        delete_empty_folders(&target_folder, &cli, &counter)?;
    }
    log!("Deleted {} empty folders", counter.load(Ordering::Relaxed));
    Ok(())
}

fn delete_empty_folders(path: &Path, cli: &Cli, counter: &AtomicU32) -> Result<()> {
    if !path.is_dir() {
        return Ok(());
    }

    let mut is_empty = true;
    for entry in fs::read_dir(path)? {
        if entry.is_err() {
            log!("Failed to read entry in {}: {:?}", path.display(), entry.err().unwrap());
            continue;
        }

        let entry = entry?;
        let entry_path = entry.path();
        let file_type = entry.file_type()?;

        if !cli.follow_symbolic_links && file_type.is_symlink() {
            is_empty = false;
            continue;
        }

        if entry_path.is_dir() {
            // Recursively delete empty subfolders
            delete_empty_folders(&entry_path, cli, counter)?;
        } else {
            // If there's a file, the folder is not empty
            is_empty = false;
        }
    }

    // If the folder is empty after processing, delete it
    if is_empty && path.read_dir()?.next().is_none() {
        delete_empty_folder(path, cli, counter)?;
    }
    Ok(())
}

fn delete_empty_folder(path: &Path, cli: &Cli, counter: &AtomicU32) -> Result<()> {
    if !path.exists() {
        log!("Warning: tried to delete a path that does not exist: {}", path.display());
        return Ok(());
    }

    let count = counter.fetch_add(1, Ordering::Relaxed);
    if cli.dry_run {
        log!("{}. Would delete empty folder: {}", count + 1, path.display());
    } else {
        log!("{}. Deleting empty folder: {}", count + 1, path.display());
        trash::delete(path)?;
    }
    Ok(())
}