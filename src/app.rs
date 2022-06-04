//! The `App` is responsible for owning and manipulating a Game of Life [`Universe`].

use crate::universe::{Universe, DEFAULT_CELL_SIZE};

pub struct App {
	pub next_cell_size: u8,
	pub running: bool,
	pub universe: Universe,
}

impl App {
	/// Instantiate a new `App`.
	#[must_use]
	pub fn new() -> Self {
		Self {
			next_cell_size: DEFAULT_CELL_SIZE,
			running: false,
			universe: Universe::new(),
		}
	}

	/// Reset the grid using the next cell size.
	pub fn new_grid(&mut self) {
		self.universe
			.resize(self.universe.cols, self.universe.rows, self.next_cell_size);
	}

	/// Start or stop the app.
	pub fn toggle_running(&mut self) {
		// TODO - move the whole universe to a separate thread?
		self.running = !self.running;
	}
}
