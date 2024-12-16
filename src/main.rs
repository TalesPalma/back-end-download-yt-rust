use std::process::Command;

use interface::menu::menu_interface;
use service::service_donwload::baixar_playlist;
mod interface;
mod service;

#[tokio::main]
async fn main() {
    init_program_menu().await;
}

async fn init_program_menu() {
    let mut url_playlist = String::new();
    menu_interface();
    std::io::stdin().read_line(&mut url_playlist).unwrap();
    let url_playlist = url_playlist.trim();
    baixar_playlist(url_playlist).await.unwrap();
    Command::new("clear").status().unwrap();
    println!("Playlist baixada com sucesso!");
}
