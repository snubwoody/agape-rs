use winresource::WindowsResource;

/// Currently this just bundles the icon into the executable on windows.
/// But it will be used for all 'build-type' functionality.
///
/// Use this in your `build.rs` file.
///
/// ```no_run
/// use agape::build;
///
/// pub fn main(){
///     agape::build()
/// }
/// ```
pub fn build() {
    #[cfg(windows)]
    set_windows_icon();
}

#[cfg(windows)]
fn set_windows_icon() {
    let mut res = WindowsResource::new();
    res.set_icon("icon.ico");
    res.compile().expect("Failed to compile windows resources");
}
