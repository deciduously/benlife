//! The `App` is responsible for owning and manipulating a Game of Life [`Universe`].

use parking_lot::RwLock;

use crate::universe::{Generation, Universe};
use crossbeam::channel;
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};

pub struct App {
	running: Arc<AtomicBool>,
	ui_receiver: channel::Receiver<Message>,
	pub universe: Universe,
}

impl App {
	/// Instantiate a new `App`.
	#[must_use]
	pub fn new(
		ui_receiver: channel::Receiver<Message>,
		running: Arc<AtomicBool>,
		shared_map: Arc<RwLock<Generation>>,
	) -> Self {
		Self {
			running,
			ui_receiver,
			universe: Universe::new(shared_map),
		}
	}

	/// Reset the grid using the next cell size.
	pub fn new_grid(&mut self, next_cell_size: u8) {
		self.universe
			.resize(self.universe.cols, self.universe.rows, next_cell_size);
	}

	/// Run the app forever.  Use messages to start or stop the application.
	pub fn run(&mut self) {
		loop {
			// Check if there was a UI interaction.
			if let Ok(msg) = self.ui_receiver.try_recv() {
				match msg {
					Message::AdvanceOne => {
						// Don't bother if the user clicks while the app is running.
						if !self.running.load(Ordering::Relaxed) {
							self.universe.advance_generation();
						}
					},
					Message::Clear => self.universe.clear(),
					Message::NewGrid(cell_size) => self.new_grid(cell_size),
					Message::Shutdown => break,
				}
			}
			// If we're in run mode, step forward.
			if self.running.load(Ordering::Relaxed) {
				self.universe.advance_generation();
			}
		}
	}
}

/// The possible messages the UI can send.
pub enum Message {
	AdvanceOne,
	Clear,
	NewGrid(u8),
	Shutdown,
}
