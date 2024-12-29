
/// Contains all the icons from the [feather icons](https://feathericons.com/) library
// TODO add this behind a feature flag since it increased binary size
pub mod feather_icons{
	use helium_macros::include_icons;

	// The path is relative to the root crate
	include_icons!("./helium/icons/feather-icons");

}

