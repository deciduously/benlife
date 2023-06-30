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
	sender: channel::Sender<ToUiMeg>,
	receiver: channel::Receiver<FromUiMsg>,
	pub universe: Universe,
}

impl App {
	/// Instantiate a new `App`.
	#[must_use]
	pub fn new(
		sender: channel::Sender<ToUiMeg>,
		receiver: channel::Receiver<FromUiMsg>,
		running: Arc<AtomicBool>,
		shared_map: Arc<RwLock<Generation>>,
	) -> Self {
		Self {
			running,
			sender,
			receiver,
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
			if let Ok(msg) = self.receiver.try_recv() {
				match msg {
					FromUiMsg::AdvanceOne => {
						// Don't bother if the user clicks while the app is running.
							self.universe.advance_generation();
					},
					FromUiMsg::Clear => self.universe.clear(),
					FromUiMsg::NewGrid(cell_size) => self.new_grid(cell_size),
					FromUiMsg::Shutdown => break,
				}
			}
			// If we're in run mode, step forward.
			if self.running.load(Ordering::SeqCst) {
				self.universe.advance_generation();
				let result = self.sender.try_send(ToUiMeg::Repaint);
				if let Err(e) = result {
					eprintln!("[{:?}] Failed to send repaint message to UI thread: {e}.", std::time::SystemTime::now());
				}
			}
		}
	}
}

/// The possible messages the UI can send.
pub enum FromUiMsg {
	AdvanceOne,
	Clear,
	NewGrid(u8),
	Shutdown,
}

pub enum ToUiMeg {
	Repaint
}