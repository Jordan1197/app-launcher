#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico"); // Path to your icon
    res.compile().unwrap();
}
