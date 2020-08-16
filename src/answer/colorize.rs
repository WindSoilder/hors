use ansi_term::Color;
use ansi_term::Color::{Fixed, RGB};

use std::env;
use syntect::easy::HighlightLines;
use syntect::highlighting::{self, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

/// make code block colorized.
///
/// Note that this function should only accept code block.
pub fn colorize_code(code: String, possible_tags: &[String]) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax: &SyntaxReference = guess_syntax(&possible_tags, &ss);
    let mut h = HighlightLines::new(&syntax, &ts.themes["base16-eighties.dark"]);
    let mut colorized: String = String::new();

    let is_true_color = is_truecolor_terminal();
    for line in LinesWithEndings::from(code.as_str()) {
        let highlights = h.highlight(line, &ss);
        for (style, component) in highlights {
            let color = to_ansi_color(style.foreground, is_true_color);
            colorized.push_str(&color.paint(component).to_string());
        }
    }
    colorized
}

/// Convert from color information to another asci_term::Color intormation.
fn to_ansi_color(color: highlighting::Color, true_color: bool) -> ansi_term::Color {
    // Note: the implementation is copied-from bat repo:
    // https://github.com/sharkdp/bat/blob/3a85fd767bd1f03debd0a60ac5bc08548f95bc9d/src/terminal.rs#L6
    if color.a == 0 {
        // Themes can specify one of the user-configurable terminal colors by
        // encoding them as #RRGGBBAA with AA set to 00 (transparent) and RR set
        // to the 8-bit color palette number. The built-in themes ansi-light,
        // ansi-dark, base16, and base16-256 use this.
        match color.r {
            // For the first 8 colors, use the Color enum to produce ANSI escape
            // sequences using codes 30-37 (foreground) and 40-47 (background).
            // For example, red foreground is \x1b[31m. This works on terminals
            // without 256-color support.
            0x00 => Color::Black,
            0x01 => Color::Red,
            0x02 => Color::Green,
            0x03 => Color::Yellow,
            0x04 => Color::Blue,
            0x05 => Color::Purple,
            0x06 => Color::Cyan,
            0x07 => Color::White,
            // For all other colors, use Fixed to produce escape sequences using
            // codes 38;5 (foreground) and 48;5 (background). For example,
            // bright red foreground is \x1b[38;5;9m. This only works on
            // terminals with 256-color support.
            //
            // TODO: When ansi_term adds support for bright variants using codes
            // 90-97 (foreground) and 100-107 (background), we should use those
            // for values 0x08 to 0x0f and only use Fixed for 0x10 to 0xff.
            n => Fixed(n),
        }
    } else if true_color {
        RGB(color.r, color.g, color.b)
    } else {
        Fixed(ansi_colours::ansi256_from_rgb((color.r, color.g, color.b)))
    }
}

fn guess_syntax<'a>(possible_tags: &[String], ss: &'a SyntaxSet) -> &'a SyntaxReference {
    for tag in possible_tags {
        let syntax = ss.find_syntax_by_token(tag.as_str());
        if let Some(result) = syntax {
            return result;
        }
    }
    ss.find_syntax_plain_text()
}

/// Return true if the current running terminal support true color.
fn is_truecolor_terminal() -> bool {
    // Note: the implementation comes from bat repo:
    // https://github.com/sharkdp/bat/blob/3a62e3d18835dce57294e5cec48e9d878351629b/src/bin/bat/app.rs#L27
    // and basic information can refer to:
    // https://unix.stackexchange.com/questions/450365/check-if-terminal-supports-24-bit-true-color
    env::var("COLORTERM")
        .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
        .unwrap_or(false)
}
