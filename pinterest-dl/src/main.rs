use std::env;
use std::path::PathBuf;

use pinterest_dl_core::{download_video, extract_download_url, filename_from_url};

fn main() {
    let mut args = env::args();
    let bin_name = args.next().unwrap_or_default();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        eprintln!("Usage: {bin_name} <pin-url> [destination-directory]");
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

struct Config {
    pin_url: String,
    dest_dir: PathBuf,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let pin_url = args.next().ok_or("Pin URL is required")?;
        let dest_dir = args
            .next()
            .map(PathBuf::from)
            .unwrap_or_else(|| env::current_dir().unwrap_or_default());

        Ok(Config { pin_url, dest_dir })
    }
}

fn run(config: Config) -> Result<(), String> {
    println!("Extracting download URL...");
    let download_url = extract_download_url(&config.pin_url).map_err(|e| format!("{e:?}"))?;

    let filename = filename_from_url(&download_url).unwrap_or_else(|| "video.mp4".to_owned());
    let dest = config.dest_dir.join(&filename);

    println!("Downloading to: {}", dest.display());
    download_video(&download_url, &dest).map_err(|e| format!("{e:?}"))?;

    println!("Download completed: {}", dest.display());
    Ok(())
}
