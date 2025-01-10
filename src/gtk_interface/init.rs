extern crate gtk;

use std::process::Command;
use std::rc::Rc;

use gtk::glib::{MainContext, Propagation};
use gtk::{prelude::*, Statusbar};
use gtk::{Button, Entry, Window, WindowType};

use crate::service::service_donwload::baixar_playlist;

// Constantes
const MUSIC_FOLDER: &str = "/home/talespalma/Musicas/";
const WINDOW_TITLE: &str = "Baixar músicas do YouTube";
const DEFAULT_WINDOW_SIZE: (i32, i32) = (400, 70);

pub fn interface() {
    gtk::init().expect("Falha ao inicializar a interface GTK");

    let window = create_window();
    let status_bar = Rc::new(create_status_bar());
    let text_entry = Entry::new();
    let button_download = Button::new();
    let button_open_folder = Button::new();
    let button_clean_folder = Button::new();

    // labels buttons

    button_download.set_label("Baixar");
    button_open_folder.set_label("Abrir pasta");
    button_clean_folder.set_label("Limpar pasta");

    button_open_folder.set_sensitive(false);

    let layout = create_layout(
        &text_entry,
        &button_download,
        &status_bar,
        &button_open_folder,
        &button_clean_folder,
    );

    window.add(&layout);

    setup_event_handlers(
        &button_download,
        &button_open_folder,
        &button_clean_folder,
        &text_entry,
        Rc::clone(&status_bar),
    );

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Propagation::Stop
    });

    window.show_all();
    gtk::main();
}

fn create_window() -> Window {
    let window = Window::new(WindowType::Toplevel);
    window.set_title(WINDOW_TITLE);
    window.set_default_size(DEFAULT_WINDOW_SIZE.0, DEFAULT_WINDOW_SIZE.1);
    window
}

fn create_status_bar() -> Statusbar {
    let status_bar = Statusbar::new();
    status_bar.push(0, "Bem vindo!");
    status_bar
}

fn create_layout(
    text_entry: &Entry,
    button_download: &Button,
    status_bar: &Statusbar,
    button_open_folder: &Button,
    button_clean_folder: &Button,
) -> gtk::Box {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    vbox.pack_start(text_entry, true, false, 0);
    vbox.pack_start(button_download, true, false, 0);
    vbox.pack_start(status_bar, true, false, 0);
    vbox.pack_end(button_open_folder, true, false, 0);
    vbox.pack_end(button_clean_folder, true, false, 0);

    vbox
}

fn setup_event_handlers(
    button_download: &Button,
    button_open_folder: &Button,
    button_clean_folder: &Button,
    text_entry: &Entry,
    status_bar: Rc<Statusbar>,
) {
    // Handler para o botão de abrir pasta
    {
        let button_open_folder = button_open_folder.clone();
        button_open_folder.connect_clicked(move |_| {
            if let Err(e) = Command::new("xdg-open").arg(MUSIC_FOLDER).spawn() {
                eprintln!("Erro ao abrir pasta: {}", e);
            }
        });
    }

    // Handler para o botão de limpar pasta
    {
        let status_bar_clone = Rc::clone(&status_bar);
        button_clean_folder.connect_clicked(move |_| {
            if let Err(e) = Command::new("rm").arg("-rf").arg(MUSIC_FOLDER).spawn() {
                eprintln!("Erro ao limpar pasta: {}", e);
            } else {
                modificar_status_bar("Sucesso ao limpar", Rc::clone(&status_bar_clone));
            }
        });
    }

    // Handler para o botão de download
    {
        let text_entry = text_entry.clone();
        let status_bar_clone = Rc::clone(&status_bar);
        let button_open_folder = button_open_folder.clone();

        button_download.connect_clicked(move |button| {
            let url_text = text_entry.text().to_string();
            println!("URL: {}", url_text);

            button.set_sensitive(false);
            status_bar_clone.push(0, "Baixando música...");

            let status_bar_async = Rc::clone(&status_bar_clone);
            let button_clone = button.clone();
            let button_open_folder_clone = button_open_folder.clone(); // Clone aqui para evitar o erro

            MainContext::default().spawn_local(async move {
                match chamar_funcao_baixar_musica(url_text).await {
                    Ok(_) => {
                        modificar_status_bar("Sucesso ao baixar", Rc::clone(&status_bar_async));
                        button_open_folder_clone.set_sensitive(true); // Usa o clone aqui
                    }
                    Err(_) => {
                        modificar_status_bar(
                            "Erro ao baixar: Tente novamente",
                            Rc::clone(&status_bar_async),
                        );
                        button_clone.set_sensitive(true);
                    }
                }
            });
        });
    }
}
async fn chamar_funcao_baixar_musica(url: String) -> Result<(), ()> {
    baixar_playlist(&url).await.map_err(|_| ())
}

fn modificar_status_bar(status_msg: &str, status_bar: Rc<Statusbar>) {
    status_bar.push(0, status_msg);
}
