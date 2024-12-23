extern crate gtk;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::{MainContext, Propagation};
use gtk::{prelude::*, Statusbar};
use gtk::{Button, Entry, Window, WindowType};

use crate::service::service_donwload::baixar_playlist;

enum Status {
    Ok,
    NotFound,
    Progress,
    Error,
}

pub fn interface() {
    //init interface
    gtk::init().expect("Failed with initialization interface gtk");

    // windows create
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Baixar musicas do youtube");
    window.set_default_size(400, 70);

    //button create
    let button = Button::new();
    button.set_label("Baixar");

    // Criação da barra de status
    let status_bar = Statusbar::new();
    // entry create
    let text_entry = Entry::new();

    // create layout box
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    vbox.pack_start(&text_entry, true, false, 0);
    vbox.pack_start(&button, true, false, 0);
    vbox.pack_start(&status_bar, true, false, 0);

    // add layout to window
    window.add(&vbox);

    // click button baixar
    button.connect_clicked(move |_| {
        let url_text = text_entry.text().to_string();
        println!("URL {}", url_text);

        let value_bar = status_bar.clone();
        MainContext::default().spawn_local(async move {
            match chamar_funcao_baixar_musica(url_text).await {
                Ok(_) => modificar_status_bar(Status::Ok, value_bar),
                Err(_) => modificar_status_bar(Status::Error, value_bar),
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

fn modificar_status_bar(status: Status, status_bar: Statusbar) {
    match status {
        Status::Ok => status_bar.push(0, "Baixado com sucesso!"),
        Status::NotFound => status_bar.push(0, "URL não encontrada!"),
        Status::Progress => status_bar.push(0, "Baixando..."),
        Status::Error => status_bar.push(0, "Erro ao baixar!"),
    };
}
