use {
    gtk::{
        gio::{prelude::*, resources_register, Resource},
        glib::Bytes,
        prelude::*,
        Application, CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
    },
    ui::Ui,
};

include!("../resource.rs");

mod error;
mod macros;
mod ui;

const RESOURCE_BYTES: &[u8] = include_bytes!("../out/assets.gresource");

fn main() -> error::Result<()> {
    // Load resources
    resources_register(&Resource::from_data(&Bytes::from_static(RESOURCE_BYTES))?);

    gtk::init()?;

    // Load CSS
    let provider = CssProvider::new();
    provider.load_from_resource(resource!("css/app.css"));
    StyleContext::add_provider_for_screen(
        &gtk::gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Create app
    let app = Application::new(Some(&app_id()), Default::default());

    // Create ui
    let ui_ref = Ui::new();

    app.connect_activate(move |app| {
        ui_ref.set_app(app);
        ui_ref.show();
    });

    app.run();

    Ok(())
}

fn app_id() -> String {
    let id = resource!("")[1..].replace('/', ".");
    String::from(&id[..(id.len() - 1)])
}
