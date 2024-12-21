use gtk_interface::init;
mod gtk_interface;
mod interface;
mod service;

#[tokio::main]
async fn main() {
    init::interface();
}
