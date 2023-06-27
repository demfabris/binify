use std::ffi::OsString;
use std::os::unix::fs::PermissionsExt;
use std::{fs, io, path};

use clap::Parser;
use serde_json::Value;

mod error;
mod parser;

use error::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file(s)
    #[arg(name = "FILE")]
    file: Vec<OsString>,
    /// Place the output binaries at `PATH`. Defaults to current directory
    #[arg(short, long, name = "PATH")]
    output: Option<OsString>,
    /// Wether or not to make the binaries executable (`chmod +x`). Defaults to true
    #[arg(short, long, default_value_t = true)]
    chmod: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for file in &args.file {
        let meta = fs::metadata(file)?;

        if !meta.is_file() {
            return Err(Error::NotAFile);
        }

        let ext = path::Path::new(&file)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        match ext {
            "json" => {
                let f = fs::File::open(file)?;
                let reader = io::BufReader::new(f);
                let data: Value = serde_json::from_reader(reader)?;
                let mut pairs: Vec<(String, Value)> = Vec::new();
                dfs_json(&data, &mut pairs, "");

                for (key, value) in pairs {
                    spawn_binary(&key, value.as_str().unwrap_or_default(), &args)?;
                }
            }
            "yaml" | "yml" => {
                unimplemented!("YAML support is not implemented yet")
            }
            // Assuming it's a .env like file
            _ => {
                let lines = parser::from_file(file)?;
                for line in lines {
                    let (key, value) = line?;
                    spawn_binary(&key, &value, &args)?;
                }
            }
        }
    }
    Ok(())
}

fn dfs_json(data: &Value, pairs: &mut Vec<(String, Value)>, parent: &str) {
    match data {
        _map if data.is_object() => {
            if let Some(map) = data.as_object() {
                for (key, value) in map {
                    if value.is_object() || value.is_array() {
                        dfs_json(value, pairs, key.as_str());
                    } else {
                        pairs.push((format!("{}.{}", parent, key.as_str()), value.clone()));
                    }
                }
            }
        }
        _array if data.is_array() => {
            println!("{:?}", _array);
            if let Some(array) = data.as_array() {
                for (idx, value) in array.iter().enumerate() {
                    dfs_json(value, pairs, format!("{}.{}", parent, idx).as_str());
                }
            }
        }
        _string if data.is_string() => {
            if let Some(string) = data.as_str() {
                pairs.push((parent.to_owned(), Value::String(string.to_owned())));
            }
        }
        _ => (),
    }
}

fn binary_template(shell: &str, value: &str) -> String {
    format!(
        r#"
#!{shell}
echo {value}
    "#
    )
}

/// drwxr-xr-x
const EXECUTABLE_UNIX_MODE_BITS: u32 = 0o0040755;

/// Create a binary file with the given key and value
fn spawn_binary(key: &str, value: &str, args: &Args) -> Result<()> {
    let shell = get_shell();
    let template = binary_template(&shell, value);

    // Set the output path if the user provided some
    let output = if let Some(output) = &args.output {
        path::Path::new(output).join(path::Path::new(key))
    } else {
        path::PathBuf::from(key)
    };

    let bin = fs::File::create(output)?;
    // Set executable permissions
    if args.chmod {
        let mut perms = bin.metadata()?.permissions();
        perms.set_mode(EXECUTABLE_UNIX_MODE_BITS);
        bin.set_permissions(perms)?;
    }
    fs::write(key, template)?;
    Ok(())
}

/// The laziest way to get your system shell
fn get_shell() -> String {
    std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_owned())
}
