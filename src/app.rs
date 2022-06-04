//! The `App` is responsible for owning and manipulating a Game of Life [`Universe`].

use crate::universe::{Universe, DEFAULT_CELL_SIZE};

// TODO - this seems kind of useless?  Keep it for now, might be useful to add another renderer.

pub struct App {
	pub next_cell_size: u8,
	pub universe: Universe,
}

impl App {
	/// Instantiate a new `App`.
	#[must_use]
	pub fn new() -> Self {
		Self {
			next_cell_size: DEFAULT_CELL_SIZE,
			universe: Universe::new(),
		}
	}

	/// Reset the grid using the next cell size.
	pub fn new_grid(&mut self) {
		self.universe
			.resize(self.universe.cols, self.universe.rows, self.next_cell_size);
	}
}
