mod symbol;

use clap::Parser;

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
    let args = Args::parse();
    println!("args: {:#?}!", args);
}
