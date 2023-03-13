use std::fs;

use colored::Colorize;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
   #[arg(default_value("."))]
   path: String,

   #[arg(short, long)]
   all: bool,
}

fn main() {
   let args = Cli::parse();

   let mut dirs: Vec<String> = vec![];
   let mut files: Vec<String> = vec![];
   let mut symlinks: Vec<String> = vec![];
   fs::read_dir(args.path)
       .unwrap()
       .into_iter()
       .for_each(|el| {
           let file = el.unwrap();
           if file.file_type().unwrap().is_dir() {
               dirs.push(file.file_name().into_string().unwrap());
           } else if file.file_type().unwrap().is_symlink() {
               symlinks.push(file.file_name().into_string().unwrap());
           } else {
            files.push(file.file_name().into_string().unwrap());
           }
       });
   dirs.sort();
   files.sort();

   for dir in dirs {
       if !args.all && dir.chars().nth(0).unwrap() == '.' {
           continue;
       } else {
           println!("{}{}", dir.blue(), "/".blue());
           }
   }
   for symlink in symlinks {
       if !args.all && symlink.chars().nth(0).unwrap() == '.' {
           continue;
       } else {
       println!("{}", symlink.purple());
       }
   }
   for file in files {
       if !args.all && file.chars().nth(0).unwrap() == '.' {
           continue;
       } else {
       println!("{}", file.normal().clear());
       }
   }
}

