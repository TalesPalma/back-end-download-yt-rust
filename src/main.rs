use std::process::Command;

#[tokio::main]
async fn main() {
    let mut url_playlist = String::new();
    menu();
    std::io::stdin().read_line(&mut url_playlist).unwrap();
    let url_playlist = url_playlist.trim();
    baixar_playlist(url_playlist).await.unwrap();
    Command::new("clear").status().unwrap();
    println!("Playlist baixada com sucesso!");
}

async fn baixar_playlist(url_playlist: &str) -> Result<(), std::io::Error> {
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

fn menu() {
    println!("-----------------------------------------");
    println!("Bem vindo ao baixar musicas do youtube");
    println!("-----------------------------------------");
    println!("Digite o link da playlist");
}
