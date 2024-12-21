extern crate gtk;
use std::sync::{Arc, Mutex};

use gtk::glib::{MainContext, Propagation};
use gtk::prelude::*;
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
    let button = Button::new();
    button.set_label("Baixar");

    // entry create
    let text_entry = Entry::new();

    // create layout box
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    vbox.pack_start(&text_entry, true, false, 0);
    vbox.pack_start(&button, true, false, 0);

    window.add(&vbox);

    button.connect_clicked(move |_| {
        let url_text = text_entry.text().to_string();
        println!("URL {}", url_text);

        MainContext::default().spawn_local(async move {
            chamar_funcao_baixar_musica(url_text).await;
        });
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Propagation::Stop
    });

    // set window and button
    window.show_all();
    gtk::main();
}

async fn chamar_funcao_baixar_musica(url: String) {
    baixar_playlist(url.as_str()).await.unwrap()
}
