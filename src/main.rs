// vim:set expandtab shiftwidth=4 filetype=rust:

// 
// 
// ~chewygumxx/print-quote.git
// ::: :/src/main.rs
// 
// 

//
// Parses a quotes.json file, formats context, prints accordingly
// Utilised as a shell initalisation splash.
//

use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{fs, error::Error};
use owo_colors::{OwoColorize, DynColors};
use std::env;
use std::path::PathBuf;
use dirs_next::home_dir;

#[derive(Debug, Deserialize)]
struct QuotePart {
    style: Option<String>,
    #[serde(default)]
    newline: bool,
    #[serde(default)]
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Attribution {
    entity: String,
    source: String,
}

#[derive(Debug, Deserialize)]
struct Quote {
    quote: Vec<QuotePart>,
    attribution: Attribution,
}

trait Wrap {
    fn wrap(&self, width: usize) -> String;
}
impl Wrap for str {
    fn wrap(&self, width: usize) -> String {
        let mut result = String::new();
        let mut line_len = 0;

        for word in self.split_whitespace() {
            let word_len = word.len();

            if line_len + word_len + if line_len > 0 { 1 } else { 0 } > width {
                if !result.is_empty() {
                    result.push('\n');
                }
                result.push_str(word);
                line_len = word_len;
            } else {
                if line_len > 0 {
                    result.push(' ');
                    line_len += 1;
                }
                result.push_str(word);
                line_len += word_len;
            }
        }

        result
    }
}

fn get_quotes_path() -> Option<PathBuf> {
    // Resolve quotes filepath from XDG_CONFIG_HOME, else ~/.config
    let config_dir = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut home = home_dir().expect("Failed to get home directory");
            home.push(".config");
            home
        });

    let mut quotes_path = config_dir;
    quotes_path.push("print-quote");
    quotes_path.push("quotes.json");

    Some(quotes_path)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Deserialise quotes
    let path = get_quotes_path().ok_or_else(|| std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "I can't see shit (unable to resolve quotes filepath)",
    ))?;
    let data = fs::read_to_string(&path)?;
    let quotes: Vec<Quote> = serde_json::from_str(&data)?;

    let mut rng = rand::thread_rng();
    if let Some(selected) = quotes.choose(&mut rng) {
        // Quote
        for segment in &selected.quote {
            let text = segment
                .content
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("")
                .wrap(80);

            let color_standard = DynColors::Rgb( 15, 225, 146); // #0fe192
            let color_context  = DynColors::Rgb(198, 196,  94); // #c6c45e
            let color_emphasis = DynColors::Rgb( 15, 255, 114); // #0fff72

            let styled = match segment.style.as_deref() {
                Some("emphasis") => format!("{}", text.color(color_emphasis).bold()),
                Some("context")  => format!("{}", text.color(color_context).italic()),
                Some("standard") => format!("{}", text.color(color_standard)),
                _ => text.to_string(),
            };
        
            if segment.newline {
                println!("{styled}");
            } else {
                print!("{styled} ");
            }
        }

        // Attribution
        let color_tilde  = DynColors::Rgb(116,   8, 207); // #7408cf
        let color_entity = DynColors::Rgb( 95, 149, 250); // #5f95fa
        let color_source = DynColors::Rgb(116,   8, 207); // #7408cf
        println!(
            "\n    {} {} {}",
            "~".color(color_tilde),
            selected.attribution.entity.color(color_entity), 
            selected.attribution.source.color(color_source).italic(),
        );
    } else {
        eprintln!("This bitch empty, yeeeeet (quotes filepath resolved, nothing in it)");
    }

    Ok(())
}
