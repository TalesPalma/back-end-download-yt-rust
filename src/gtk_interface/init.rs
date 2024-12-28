extern crate gtk;

use std::process::Command;

use gtk::glib::{MainContext, Propagation};
use gtk::{prelude::*, Statusbar};
use gtk::{Button, Entry, Window, WindowType};

use crate::service::service_donwload::baixar_playlist;

pub fn interface() {
    //init interface
    gtk::init().expect("Failed with initialization interface gtk");

    // windows create
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Baixar musicas do youtube");
    window.set_default_size(400, 70);

    //button create
    let button_donwload = Button::new();
    button_donwload.set_label("Baixar");

    //button create open folder with musics
    let button_open_folder = Button::new();
    button_open_folder.set_label("Abrir pasta das musicas baixadas");
    button_open_folder.set_sensitive(false);

    // Criação da barra de status
    let status_bar = Statusbar::new();
    status_bar.push(0, "Bem vindo!");
    // entry create
    let text_entry = Entry::new();

    // create layout box
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    vbox.pack_start(&text_entry, true, false, 0);
    vbox.pack_start(&button_donwload, true, false, 0);
    vbox.pack_start(&status_bar, true, false, 0);
    vbox.pack_end(&button_open_folder, true, false, 0);

    // add layout to window
    window.add(&vbox);

    // click button open folder
    button_open_folder.connect_clicked(move |_| {
        let url_text = "/home/talespalma/Musicas/";
        if let Err(e) = Command::new("xdg-open").arg(url_text).spawn() {
            eprintln!("Erro ao abrir pasta {}", e)
        }
    });

    // click button baixar
    button_donwload.connect_clicked(move |e| {
        let url_text = text_entry.text().to_string();
        println!("URL {}", url_text);

        let value_bar = status_bar.clone();
        let button_open_folder_clone = button_open_folder.clone();

        e.set_sensitive(false);

        let context = MainContext::default();
        context.spawn_local(async move {
            match chamar_funcao_baixar_musica(url_text).await {
                Ok(_) => {
                    modificar_status_bar("Sucesso ao baixar", value_bar);
                    button_open_folder_clone.set_sensitive(true);
                }
                Err(_) => {
                    modificar_status_bar("Erro ao baixar: Tente novamente", value_bar);
                }
            }
        });
    });

    // close window with click close
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Propagation::Stop
    });

    // set window and button
    window.show_all();
    gtk::main();
}

async fn chamar_funcao_baixar_musica(url: String) -> Result<(), ()> {
    match baixar_playlist(url.as_str()).await {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

fn modificar_status_bar(status_msg: &str, status_bar: Statusbar) {
    status_bar.push(0, status_msg);
}
