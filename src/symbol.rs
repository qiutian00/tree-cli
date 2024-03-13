use std::fs::Metadata;
use std::io;

use term::color;

use crate::Config;

/// 横线
pub const HOR: char = '─';
/// 叉号
pub const CRO: char = '├';
/// 竖线
pub const VER: char = '│';
/// 末尾符号
pub const END: char = '└';
/// 空格
pub const SPACE: char = ' ';

pub fn set_line_prefix(symbol_switch_list: &Vec<bool>, prefix: &mut String) {
    let len = symbol_switch_list.len();
    let index = len.saturating_sub(1);
    prefix.clear();
    for symbol_switch in symbol_switch_list.iter().take(index) {
        if *symbol_switch {
            prefix.push(VER);
        } else {
            prefix.push(SPACE);
        }
        prefix.push(SPACE);
        prefix.push(SPACE);
        prefix.push(SPACE);
    }
    if let Some(symbol_switch) = symbol_switch_list.last() {
        if *symbol_switch {
            prefix.push(CRO);
        } else {
            prefix.push(END);
        }
        prefix.push(HOR);
        prefix.push(HOR);
        prefix.push(SPACE);
    }
}

pub fn print_path(
    file_name: &str,
    metadata: &Metadata,
    t: &mut Box<term::StdoutTerminal>,
    config: &Config,
) -> io::Result<()> {
    if metadata.is_dir() {
        write_color(t, config, color::BRIGHT_BLUE, file_name)
    } else if is_executable(metadata) {
        write_color(t, config, color::BRIGHT_RED, file_name)
    } else {
        write!(t, "{}", file_name)
    }
}

fn write_color(
    t: &mut Box<term::StdoutTerminal>,
    config: &Config,
    color: color::Color,
    str: &str,
) -> io::Result<()> {
    if config.colorful {
        t.fg(color)?;
    }

    write!(t, "{}", str)?;

    if config.colorful {
        t.reset()?;
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn is_executable(_metadata: &Metadata) -> bool {
    false
}

#[cfg(target_os = "linux")]
fn is_executable(metadata: &Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    let mode = metadata.permissions().mode();
    (mode & 0o100) != 0
}
