use {
    crate::{get_obj, resource},
    gtk::{prelude::*, AboutDialog, ApplicationWindow, Builder},
    std::rc::Rc,
};

#[derive(Debug)]
pub struct Ui {
    main_window: ApplicationWindow,
}

impl Ui {
    pub fn new() -> Rc<Self> {
        let b = Builder::from_resource(resource!("ui/main"));
        Rc::new(Self {
            main_window: get_obj!(b, "main-window"),
        })
    }
}
