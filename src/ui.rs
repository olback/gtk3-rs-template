use {
    crate::get_obj,
    gtk::{glib::clone, prelude::*, AboutDialog, ApplicationWindow, Builder, Button},
    std::rc::Rc,
};

#[derive(Debug)]
pub struct Ui {
    main_window: ApplicationWindow,
    about_dialog: AboutDialog,
    show_about_dialog_btn: Button,
}

impl Ui {
    pub fn new() -> Rc<Self> {
        let b = Builder::from_resource(resource!("ui/main"));
        let inner = Rc::new(Self {
            main_window: get_obj!(b, "main-window"),
            about_dialog: get_obj!(b, "about-dialog"),
            show_about_dialog_btn: get_obj!(b, "show-about-dialog-btn"),
        });

        inner
            .show_about_dialog_btn
            .connect_clicked(clone!(@strong inner => move |_| {
                match inner.about_dialog.run() {
                    _ => inner.about_dialog.hide()
                }
            }));

        inner
    }

    pub fn set_app(&self, app: &gtk::Application) {
        self.main_window.set_application(Some(app));
    }

    pub fn show(&self) {
        self.main_window.show_all();
    }
}
