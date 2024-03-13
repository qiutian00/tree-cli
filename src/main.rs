use std::path::Path;

use clap::Parser;
use globset::Glob;

use crate::core::DirTree;

mod symbol;
mod pojo;
mod core;
mod filter;
mod file_iterator;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Show all files (include hidden files)
    #[arg(short = 'a', long = "all")]
    show_all: bool,
    /// Turn colorization off always
    #[arg(short = 'C', long = "color")]
    color_on: bool,
    /// Directory you want to search
    #[arg(value_name = "DIR", default_value = ".")]
    dir: String,
    /// List only those files matching <include_pattern>
    #[arg(short = 'P', long = "pattern")]
    include_pattern: Option<String>,
    /// Descend only <level> directories deep
    #[arg(short = 'L', long = "level", default_value_t = usize::max_value())]
    max_level: usize,
}


fn main() {
    let Args { show_all, color_on, dir, include_pattern, max_level } = Args::parse();
    let path = Path::new(&dir);
    let mut mt = term::stdout().expect("Could not unwrap term::stdout.");
    let config = pojo::Config {
        colorful: color_on,
        show_all,
        max_level,
        include_glob: include_pattern.map(|pat| {
            Glob::new(pat.as_str()).expect("include_pattern is not valid").compile_matcher()
        }),
    };
    let dir_tree = DirTree::new(config, &mut mt);


    mt.fg(term::color::GREEN).unwrap();
    write!(mt, "hello, ").unwrap();

    mt.fg(term::color::RED).unwrap();
    writeln!(mt, "world!").unwrap();

    mt.reset().unwrap();
}
