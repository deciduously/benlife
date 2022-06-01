//! The `App` consists of a Game of Life [`Universe`] and some user-tweakable [`Controls`].

use crate::{controls::Controls, universe::Universe};

#[derive(Default)]
pub struct App {
	controls: Controls,
	universe: Universe,
}

impl App {
	/// Instantiate a new `App`.
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}
}
