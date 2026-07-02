use std::path::PathBuf;

use crate::downloader::{download_video, extract_download_url, filename_from_url};

/// Abstrai o processamento do formulário enviado pela interface.
///
/// Em produção, a implementação padrão extrai o link de download a partir da
/// URL do pin e baixa o vídeo para o diretório escolhido pelo usuário.
pub trait Output {
    fn handle_submission(&mut self, pin_url: &str, directory_path: &str);
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ConsoleOutput;

impl Output for ConsoleOutput {
    fn handle_submission(&mut self, pin_url: &str, directory_path: &str) {
        let pin_url = pin_url.to_owned();
        let directory_path = directory_path.to_owned();
        std::thread::spawn(move || {
            match extract_download_url(&pin_url) {
                Ok(download_url) => {
                    println!("Link de download extraído: {download_url}");

                    let filename = filename_from_url(&download_url)
                        .unwrap_or_else(|| "video.mp4".to_owned());
                    let dest = PathBuf::from(&directory_path).join(&filename);

                    println!("Baixando para {}...", dest.display());

                    match download_video(&download_url, &dest) {
                        Ok(()) => println!("Download concluído: {}", dest.display()),
                        Err(e) => eprintln!("Erro ao baixar vídeo: {e:?}"),
                    }
                }
                Err(e) => eprintln!("Erro ao extrair link: {e:?}"),
            }
        });
    }
}
