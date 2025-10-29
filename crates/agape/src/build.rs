/// Currently this just bundles the icon into the executable on windows.
/// But it will be used for all 'build-type' functionality.
///
/// Use this in your `build.rs` file.
pub fn build() {
    #[cfg(windows)]
    set_windows_icon();
}

#[cfg(windows)]
fn set_windows_icon() {
    use winresource::WindowsResource;
    let mut res = WindowsResource::new();
    res.set_icon("icon.ico");
    res.compile().expect("Failed to compile windows resources");
}
