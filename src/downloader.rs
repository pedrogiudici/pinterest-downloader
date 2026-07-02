/// Extrai o link de download de um vídeo do Pinterest a partir da URL do pin.
///
/// Faz uma requisição HTTP para a página do pin e procura por um link `.mp4`
/// no HTML retornado.
use regex::Regex;
use std::io::Read;

/// Erros que podem ocorrer durante a extração ou download do vídeo.
#[derive(Debug)]
pub enum DownloadError {
    /// A requisição HTTP falhou.
    Request(String),
    /// Não foi possível encontrar um link de vídeo na página.
    VideoNotFound,
    /// Falha ao fazer download do vídeo.
    DownloadFailed(String),
    /// Erro de E/S ao salvar o arquivo.
    IoError(String),
}

/// Extrai a URL de download do vídeo a partir da URL do pin do Pinterest.
///
/// # Exemplo
///
/// ```ignore
/// let url = extract_download_url("https://in.pinterest.com/pin/879890845950202614/").unwrap();
/// println!("{url}");
/// // https://v1.pinimg.com/videos/iht/720p/b7/1d/60/b71d60335f58562e9d2da8d0e06e4013.mp4
/// ```
pub fn extract_download_url(pin_url: &str) -> Result<String, DownloadError> {
    let html = fetch_page(pin_url).map_err(DownloadError::Request)?;
    find_mp4_url(&html).ok_or(DownloadError::VideoNotFound)
}

fn fetch_page(url: &str) -> Result<String, String> {
    let response = ureq::get(url)
        .set(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        )
        .call()
        .map_err(|e| format!("{e}"))?;

    let body = response
        .into_string()
        .map_err(|e| format!("erro ao ler corpo da resposta: {e}"))?;

    Ok(body)
}

fn find_mp4_url(html: &str) -> Option<String> {
    let re = Regex::new(r#""(https://[^"]+\.mp4[^"]*)""#).ok()?;
    let cap = re.captures(html)?;
    let url = cap.get(1)?.as_str().replace("\\/", "/");
    Some(url)
}

/// Extrai o nome do arquivo a partir da URL de download.
pub fn filename_from_url(url: &str) -> Option<String> {
    let last_segment = url.split('/').last().unwrap_or("");
    if last_segment.is_empty() || !last_segment.contains('.') {
        return None;
    }
    // Remove qualquer query string ou fragmento
    let name = last_segment.split('?').next().unwrap_or(last_segment);
    let name = name.split('#').next().unwrap_or(name);
    if name.is_empty() {
        None
    } else {
        Some(name.to_owned())
    }
}

/// Faz o download do vídeo a partir da URL e salva no caminho de destino.
pub fn download_video(
    download_url: &str,
    destination: &std::path::Path,
) -> Result<(), DownloadError> {
    let response = ureq::get(download_url)
        .set(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        )
        .call()
        .map_err(|e| DownloadError::DownloadFailed(format!("{e}")))?;

    let mut body = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut body)
        .map_err(|e| DownloadError::IoError(format!("erro ao ler resposta: {e}")))?;

    std::fs::write(destination, &body)
        .map_err(|e| DownloadError::IoError(format!("erro ao salvar arquivo: {e}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_mp4_url_returns_url_when_present() {
        let html = r#"<script>"url":"https://v1.pinimg.com/videos/iht/720p/b7/1d/60/b71d60335f58562e9d2da8d0e06e4013.mp4"</script>"#;
        let url = find_mp4_url(html);
        assert_eq!(
            url,
            Some(
                "https://v1.pinimg.com/videos/iht/720p/b7/1d/60/b71d60335f58562e9d2da8d0e06e4013.mp4"
                    .to_string()
            )
        );
    }

    #[test]
    fn find_mp4_url_returns_none_when_no_video() {
        let html = "<html><body>no video here</body></html>";
        let url = find_mp4_url(html);
        assert_eq!(url, None);
    }

    #[test]
    fn find_mp4_url_handles_real_sample() {
        // Trecho real do HTML do Pinterest (urls sem escape)
        let html = r#"<script>
            const __INITIAL_STATE__ = {"resources":{"data":{"https://in.pinterest.com/pin/879890845950202614/":{"data":{"pin":{"video_list":{"V_HLSV4":{"url":"https://v1.pinimg.com/videos/iht/hls/b7/1d/60/b71d60335f58562e9d2da8d0e06e4013.m3u8"},"V_720P":{"url":"https://v1.pinimg.com/videos/iht/720p/b7/1d/60/b71d60335f58562e9d2da8d0e06e4013.mp4"}}}}}}}};</script>"#;
        let url = find_mp4_url(html);
        assert_eq!(
            url,
            Some(
                "https://v1.pinimg.com/videos/iht/720p/b7/1d/60/b71d60335f58562e9d2da8d0e06e4013.mp4"
                    .to_string()
            )
        );
    }

    #[test]
    fn extract_download_url_returns_error_for_invalid_url() {
        let result = extract_download_url("https://invalid.url.xyz/");
        assert!(result.is_err());
    }

    #[test]
    fn download_error_is_debug_and_send() {
        fn assert_send<T: Send>() {}
        assert_send::<DownloadError>();
    }

    #[test]
    fn download_error_display() {
        // Apenas verifica que podemos criar os erros
        let err1 = DownloadError::Request("timeout".into());
        let err2 = DownloadError::VideoNotFound;
        assert!(format!("{err1:?}").contains("Request"));
        assert!(format!("{err2:?}").contains("VideoNotFound"));
    }

    #[test]
    fn filename_from_url_extracts_mp4_name() {
        let url =
            "https://v1.pinimg.com/videos/iht/720p/b7/1d/60/b71d60335f58562e9d2da8d0e06e4013.mp4";
        assert_eq!(
            filename_from_url(url),
            Some("b71d60335f58562e9d2da8d0e06e4013.mp4".to_owned())
        );
    }

    #[test]
    fn filename_from_url_returns_none_for_empty_path() {
        let url = "https://example.com/";
        assert_eq!(filename_from_url(url), None);
    }

    #[test]
    fn filename_from_url_returns_name_for_simple_url() {
        let url = "https://example.com/video.mp4";
        assert_eq!(filename_from_url(url), Some("video.mp4".to_owned()));
    }

    #[test]
    fn download_video_fails_on_invalid_url() {
        let result = download_video(
            "https://invalid.url.xyz/",
            std::path::Path::new("/tmp/test_download.mp4"),
        );
        assert!(result.is_err());
    }

    #[test]
    fn download_video_saves_file_when_successful() {
        // Usamos um servidor HTTP local para simular um download bem-sucedido.
        let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
        let port = server.server_addr().to_ip().unwrap().port();
        let url = format!("http://127.0.0.1:{port}/video.mp4");

        let dest = std::env::temp_dir().join("test_download_output.mp4");
        // Remove se existir
        let _ = std::fs::remove_file(&dest);

        let handle = std::thread::spawn(move || {
            let request = server.recv().unwrap();
            let response = tiny_http::Response::from_string("fake mp4 content").with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"video/mp4"[..]).unwrap(),
            );
            request.respond(response).unwrap();
        });

        let result = download_video(&url, &dest);
        handle.join().unwrap();

        assert!(result.is_ok());
        assert!(dest.exists());
        let content = std::fs::read_to_string(&dest).unwrap();
        assert_eq!(content, "fake mp4 content");

        // Limpeza
        let _ = std::fs::remove_file(&dest);
    }
}
