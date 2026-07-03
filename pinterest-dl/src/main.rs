use std::path::PathBuf;

use pinterest_dl_core::{download_video, extract_download_url, filename_from_url};

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Erro: {err}");
        eprintln!("Uso: {} <pin-url> [diretorio-destino]", std::env::args().next().unwrap_or_default());
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Erro: {e}");
        std::process::exit(1);
    }
}

struct Config {
    pin_url: String,
    dest_dir: PathBuf,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let pin_url = args.next().ok_or("URL do pin é obrigatória")?;
        let dest_dir = args
            .next()
            .map(PathBuf::from)
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

        Ok(Config { pin_url, dest_dir })
    }
}

fn run(config: Config) -> Result<(), String> {
    println!("Extraindo URL de download...");
    let download_url = extract_download_url(&config.pin_url)
        .map_err(|e| format!("{e:?}"))?;

    let filename = filename_from_url(&download_url).unwrap_or_else(|| "video.mp4".to_owned());
    let dest = config.dest_dir.join(&filename);

    println!("Baixando para: {}", dest.display());
    download_video(&download_url, &dest).map_err(|e| format!("{e:?}"))?;

    println!("Download concluido: {}", dest.display());
    Ok(())
}
