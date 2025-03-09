use clap::Parser;
use std::fs::{self, DirEntry};
use std::io;

#[derive(Parser)]
#[command(version="1.0", about = "List information about the FILEs (current directory by default).\n Sort entries alphabetically if none of -cftuvSUX nor --sort is specified", long_about = None)]
struct Args {
    /// [File]s and/or directories
    file: Option<Vec<String>>,

    /// Do not ignore entries starting with '.'
    #[arg(short, long)]
    all: bool,

    /// Use a long listing format
    #[arg(short)]
    l: bool,
}

fn gen_file_output(entry: DirEntry, long_list: bool) -> String {
    let file_name = entry.file_name().to_string_lossy().into_owned();

    // Just the file name needed if not -l
    if !long_list {
        return file_name;
    }
    let err_str = format!("pl-ls Cannot access {file_name}: No such file or directory");

    let file_type = if let Ok(file_type) = entry.file_type() {
        file_type
    } else {
        return err_str;
    };
    let metadata = if let Ok(metadata) = entry.metadata() {
        metadata
    } else {
        return err_str;
    };

    format!("perms{:?}", metadata.permissions())
}

fn gen_output(path: String, long_list: bool) {
    let entries = if let Ok(entries) = fs::read_dir(path) {
        entries
    } else {
        println!("File is not accessible");
        return;
    };

    for entry in entries {
        let entry = if let Ok(entry) = entry {
            entry
        } else {
            println!("File is not accessible");
            continue;
        };
        println!("{}", gen_file_output(entry, long_list))
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Make sure if multiple files are specified, their output is in alphabetical order
    let mut search_files: Vec<String> = args.file.unwrap_or(vec![String::from(".")]);
    search_files.sort();

    let disp_filename = search_files.len() > 1;

    for file in search_files {
        if disp_filename {
            println!("{file}:");
        }

        gen_output(file, args.l);
    }

    // let mut entries = fs::read_dir(".")?
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.
    // entries.sort();
    // for entry in entries {
    //     println!("{:?}", entry);
    // }

    // The entries have now been sorted by their path.

    Ok(())
}
