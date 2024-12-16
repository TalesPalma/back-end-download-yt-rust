use std::process::Command;

pub async fn baixar_playlist(url_playlist: &str) -> Result<(), std::io::Error> {
    let status = Command::new("yt-dlp")
        .args(&[
            "-x",
            "--audio-format",
            "mp3",
            "--yes-playlist",
            "-o",
            "~/Musicas/%(title)s.%(ext)s",
            url_playlist,
        ])
        .status()
        .expect("Erro with download playlist");

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error with download playlist",
        ))
    }
}
