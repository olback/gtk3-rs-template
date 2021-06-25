use giftwrap::Wrap;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Wrap)]
pub enum Error {
    Glib(gtk::glib::Error),
    Bool(gtk::glib::BoolError),
}
