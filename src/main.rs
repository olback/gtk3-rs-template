use {
    gtk::{
        gio::{prelude::*, Resource},
        glib::{clone, Bytes},
        prelude::*,
        Application, CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
    },
    ui::Ui,
};

mod error;
mod macros;
mod ui;

const RESOURCE_BYTES: &[u8] =
    include_bytes!(concat!("../out/", env!("CARGO_PKG_NAME"), ".gresource"));

fn main() -> error::Result<()> {
    // Load resources
    gtk::gio::resources_register(&Resource::from_data(&Bytes::from_static(RESOURCE_BYTES))?);

    gtk::init()?;

    // Load CSS
    let provider = CssProvider::new();
    provider.load_from_resource(resource!("css/app.css"));
    StyleContext::add_provider_for_screen(
        &gtk::gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    Ok(())
}
