#[macro_export]
macro_rules! resource {
    ($res:expr) => {
        // Change '/net/olback/' to whatever fits your project
        concat!("/net/olback/", env!("CARGO_PKG_NAME"), "/", $res)
    };
}
