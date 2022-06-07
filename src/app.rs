//! The `App` is responsible for owning and manipulating a Game of Life [`Universe`].

use crate::{universe::Universe, Context};
use crossbeam::channel;
use std::sync::{atomic::Ordering, Arc};

pub(crate) struct App {
	context: Arc<Context>,
	ui_receiver: channel::Receiver<Message>,
	universe: Universe,
}

impl App {
	/// Instantiate a new `App`.
	#[must_use]
	pub fn new(ui_receiver: channel::Receiver<Message>, context: Arc<Context>) -> Self {
		let universe_context = Arc::clone(&context);
		Self {
			context,
			ui_receiver,
			universe: Universe::new(universe_context),
		}
	}

	/// Reset the grid using the next cell size.
	pub fn new_grid(&mut self, next_cell_size: u8) {
		self.universe.reset(next_cell_size);
	}

	/// Run the app forever.  Use messages to start or stop the application.
	pub fn run(&mut self) {
		loop {
			// Check if there was a UI interaction.
			if let Ok(msg) = self.ui_receiver.try_recv() {
				match msg {
					Message::AdvanceOne => {
						// Don't bother if the user clicks while the app is running.
						if !self.context.running.load(Ordering::Relaxed) {
							self.universe.advance_generation();
						}
					},
					Message::Clear => self.universe.clear(),
					Message::NewGrid(cell_size) => self.new_grid(cell_size),
					Message::Shutdown => break,
				}
			}
			// If we're in run mode, step forward.
			if self.context.running.load(Ordering::Relaxed) {
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
