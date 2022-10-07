mod netscape_bookmark;

use std::{env, io, panic, path, process};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use itertools::Itertools;
use netscape_bookmark::Bookmark;
use crate::netscape_bookmark::NetscapeBookmarks;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_usage();
        process::exit(0);
    }

    panic::set_hook(Box::new(|_| {}));

    if let Err(e) = run(args) {
        eprintln!("{}", e.to_string().to_lowercase().replace("\"", ""));
        print_usage();
        process::exit(1);
    }
}

fn print_usage() {
    println!("usage: instamark export.csv");
}

fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let csv_export = args.into_iter().last().unwrap_or_else(|| String::new());
    if csv_export.is_empty() {
        return Err("no Instapaper export provided".into());
    }

    let file_path = File::open(path::Path::new(&csv_export))?;
    let mut csv_file = csv::Reader::from_reader(file_path);
    let folders = csv_file.deserialize()
        .filter_map(|record| record.ok())
        .collect::<Vec<Bookmark>>()
        .into_iter()
        .group_by(|bookmark| bookmark.folder.clone())
        .into_iter()
        .fold(HashMap::<String, Vec<Bookmark>>::new(), |mut folders, group| {
            let (folder, items) = group;
            if !folders.contains_key(&folder) {
                folders.insert(folder, items.collect());
            } else {
                folders.get_mut(&folder).and_then(|group| {
                    group.extend(items);
                    Some(group)
                });
            }
            folders
        });

    let bookmarks_html = NetscapeBookmarks::new()
        .doctype()
        .title("Instapaper")
        .header("Instapaper")
        .description_lists(folders)
        .render();

    let mut stdout = io::stdout().lock();
    stdout.write_all(bookmarks_html.as_bytes())?;
    stdout.flush()?;

    Ok(())
}
