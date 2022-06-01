//! The `App` consists of a Game of Life [`Grid`] and some user-tweakable [`Controls`].

use crate::{controls::Controls, grid::Universe};

#[derive(Default)]
pub struct App {
	controls: Controls,
	grid: Universe,
}

impl App {
	/// Instantiate a new `App`.
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}
}
