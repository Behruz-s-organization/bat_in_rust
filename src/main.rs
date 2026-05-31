use std::{fs, io::Read, io::Write};
use clap::Parser;
use std::io;
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::Style;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use two_face::theme::EmbeddedThemeName;
use owo_colors::OwoColorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command()]
    file_name: String,

    #[arg(long, short)]
    theme: Option<String>,
}



fn main() {
    let args = Args::parse();
    let file_name = args.file_name;
    let theme = args.theme.unwrap_or("gruvbox_dark".to_string());
    // let theme = args.theme;
    let mut file = match fs::File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("File or folder not found, please make sure your path is correct");
            return;
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    let lines = highlight_content(&content, Some(theme));
    for line in lines {

        io::stdout().write_all(line.as_bytes())
            .expect("Failed to write to stdout");
    }
}

fn highlight_content(content: &str, theme: Option<String>) -> Vec<String> {
    let mut lines = Vec::new();
    let ps = SyntaxSet::load_defaults_newlines();
    let theme_set = two_face::theme::extra();
    let theme = theme_set.get(get_theme(theme));

    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, theme);
    for (i, line) in LinesWithEndings::from(content).enumerate() {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        lines.push(format!("{} │ {}", format!("{:>4}", i + 1).dimmed(), escaped));
    }
    lines
}

fn get_theme(theme: Option<String>) -> EmbeddedThemeName {
    match theme.as_deref() {
        Some("gruvbox_dark") => EmbeddedThemeName::GruvboxDark,
        Some("solarized_dark") => EmbeddedThemeName::SolarizedDark,
        _ => EmbeddedThemeName::GruvboxDark,
    }
}
