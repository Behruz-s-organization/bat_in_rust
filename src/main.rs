use std::{fs, io::Read, io::Write};
use clap::Parser;
use std::io;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use anyhow;
use owo_colors::OwoColorize;

mod update;

const DEFAULT_THEME: &str = "gruvbox-dark";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command()]
    file_name: Option<String>,

    #[arg(long, short)]
    theme: Option<String>,

    #[arg(long)]
    list_themes: bool,

    #[arg(long)]
    update: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.update {
        return update::update();
    };
    let theme_set: ThemeSet = two_face::theme::extra().into();

    if args.list_themes {
        list_themes(&theme_set);
        return Ok(());
    }

    let file_name = match args.file_name {
        Some(name) => name,
        None => {
            eprintln!("error: no file given. Usage: show <FILE>  (or: show --list-themes)");
            std::process::exit(1);
        }
    };

    let file_type = file_name.split('.').last().unwrap_or("");
    let theme = resolve_theme(&theme_set, args.theme.as_deref());

    let mut file = match fs::File::open(&file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("File or folder not found, please make sure your path is correct");
            return Ok(());
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    let lines = highlight_content(&content, theme, file_type);
    for line in lines {
        io::stdout()
            .write_all(line.as_bytes())
            .expect("Failed to write to stdout");
    };
    return Ok(());
}

fn highlight_content(content: &str, theme: &Theme, file_type: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let ps = two_face::syntax::extra_newlines();

    let syntax = ps
        .find_syntax_by_extension(file_type)
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    let mut h = HighlightLines::new(syntax, theme);
    for (i, line) in LinesWithEndings::from(content).enumerate() {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        lines.push(format!("{} │ {}", format!("{:>4}", i + 1).dimmed(), escaped));
    }
    lines
}

fn resolve_theme<'a>(theme_set: &'a ThemeSet, requested: Option<&str>) -> &'a Theme {
    let name = requested.unwrap_or(DEFAULT_THEME);

    if let Some(theme) = lookup_theme(theme_set, name) {
        return theme;
    }

    if requested.is_some() {
        eprintln!(
            "warning: unknown theme '{}', falling back to '{}'. Run with --list-themes to see options.",
            name, DEFAULT_THEME
        );
    }
    lookup_theme(theme_set, DEFAULT_THEME)
        .expect("default theme should always exist in two-face theme set")
}

fn lookup_theme<'a>(theme_set: &'a ThemeSet, name: &str) -> Option<&'a Theme> {
    if let Some(theme) = theme_set.themes.get(name) {
        return Some(theme);
    }

    let normalized = normalize(name);
    for (key, theme) in &theme_set.themes {
        if normalize(key) == normalized {
            return Some(theme);
        }
    }

    None
}

fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn list_themes(theme_set: &ThemeSet) {
    println!("Available themes (use with --theme <name>):\n");
    for name in theme_set.themes.keys() {
        if name == DEFAULT_THEME {
            println!("  {name}  (default)");
        } else {
            println!("  {name}");
        }
    }
}
