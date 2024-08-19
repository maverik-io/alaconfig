use std::fs;
use std::io::Write;
use std::process::exit;

use crate::{Getter, Setter};

fn write_file(out: String) {
    let path = dirs::home_dir()
        .unwrap()
        .join(".config/alacritty/alacritty.toml");
    println!("{out}");
    print!("Write? (y/n):");
    std::io::stdout().flush().unwrap();
    let mut confirmation = String::new();
    std::io::stdin()
        .read_line(&mut confirmation)
        .expect("Error reading from input");
    let confirmation = confirmation.chars().next().unwrap();
    if confirmation == 'y' {
        fs::write(path, out).expect("Could not write");
    }
}

fn title(input: String) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn collapse(input: Vec<String>) -> String {
    let mut out = String::new();
    for s in input {
        out.push_str(s.as_str());
        out.push(' ');
    }
    out
}

fn set_val(item: &str, value: String) -> String {
    let path = dirs::home_dir()
        .unwrap()
        .join(".config/alacritty/alacritty.toml");

    let conffile = fs::read_to_string(&path).unwrap();

    let mut out = String::new();

    for line in conffile.lines() {
        if line.starts_with(item) {
            out.push_str(format!("{item} = {value}").as_str());
        } else {
            out.push_str(line);
        }
        out.push('\n')
    }
    out
}

fn get_val(item: &str) -> String {
    let conffile = fs::read_to_string(
        dirs::home_dir()
            .unwrap()
            .join(".config/alacritty/alacritty.toml"),
    )
    .unwrap();
    for line in conffile.lines() {
        if line.starts_with(item) {
            return format!("{line}");
        }
    }
    return "".to_string();
}

pub fn handle_get(getter: Getter) {
    match getter.item.as_str() {
        "blur" => println!("{}", get_val("blur")),
        "font" => println!("{}", get_val("family")),
        "size" => println!("{}", get_val("size")),
        "theme" => {
            let _in = get_val("import");
            let mut _out = String::new();
            let mut recording = false;
            for i in _in.chars() {
                if i == '"' {
                    recording = !recording;
                } else if recording {
                    _out.push(i);
                }
            }
            _out = title(
                _out.split('/')
                    .last()
                    .unwrap()
                    .split('.')
                    .next()
                    .unwrap()
                    .replace("-", " "),
            );
            println!("{_out}");
        }
        "opacity" => println!("{}", get_val("opacity")),

        _ => {
            println!("Error: Invalid option :( ");
            exit(1);
        }
    };
}

pub fn handle_set(setter: Setter) {
    match setter.item.as_str() {
        "blur" => {
            let value = collapse(setter.value).trim().to_string();
            if &value == "true" || &value == "false" {
                let out = set_val("blur", value);
                write_file(out)
            } else {
                println!("Error: Expected `true` or `false, got `{}`", value);
                exit(1);
            }
        }
        "font" => {
            let value = format!("\"{}\"", title(collapse(setter.value)))
                .trim()
                .to_string();

            let out = set_val("family", value);
            write_file(out)
        }
        "size" => {
            let value = collapse(setter.value);
            let out = set_val("size", value);
            write_file(out)
        }
        "theme" => {
            let value = collapse(setter.value).trim().to_string();
            let value = format!(
                "[ \"~/.config/alacritty/themes/{}.toml\",]",
                value.to_lowercase().replace(" ", "-"),
            );
            let out = set_val("import", value);
            write_file(out);
        }
        "opacity" => {
            let mut value = collapse(setter.value).trim().to_string();
            match value.parse::<f32>() {
                Ok(val) => {
                    if val > 1.0 {
                        println!("Error: Opacity can only be between 0 and 1");
                        exit(1);
                    } else {
                        value = val.to_string();
                    }
                }
                Err(_e) => {
                    println!("Error: Invalid float");
                    exit(1);
                }
            }
            let out = set_val("opacity", value.to_string());
            write_file(out);
        }

        _ => {
            println!("Error: Invalid option :( ");
            exit(1);
        }
    }
}
