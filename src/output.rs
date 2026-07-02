use crate::downloader::extract_download_url;

/// Abstrai o processamento do formulário enviado pela interface.
///
/// Em produção, a implementação padrão extrai o link de download a partir da
/// URL do pin e também printa o path escolhido pelo usuário.
pub trait Output {
    fn handle_submission(&mut self, pin_url: &str, directory_path: &str);
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ConsoleOutput;

impl Output for ConsoleOutput {
    fn handle_submission(&mut self, pin_url: &str, directory_path: &str) {
        match extract_download_url(pin_url) {
            Ok(download_url) => println!("{download_url}"),
            Err(e) => eprintln!("Erro ao extrair link: {e:?}"),
        }

        println!("{directory_path}");
    }
}
