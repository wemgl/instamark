# Instamark
Converts an Instapaper CSV export of bookmarks into the Netscape Bookmark file format.

## Usage
Login to your Instapaper account on a desktop computer (the CSV export functionality isn't available
on mobile). Click your username in the top-right hand corner of the screen, then select settings.
Select "Download .CSV file" in the Export section at the bottom of the page. Navigate to the location
the export was saved to in a terminal window and then run:

```shell
instamark instapaper-export.csv > instapaper-bookmarks.html
```

Your exported Instapaper data is now saved in the `instapaper-bookmarks.html` file, which you can
import into Safari, Chrome, Firefox, or any other browser which supports the Netscape bookmark file
format.

## Installation
This GitHub repository contains executable binaries for macOS, Linux, and Windows in the Release
section for download.

Optionally, you can download the source code from this repository, navigate to the root directory, and run (assuming you've installd [Rust](https://www.rust-lang.org/tools/install)) to install `instamark` from scratch.
```shell
cargo build --release
```
