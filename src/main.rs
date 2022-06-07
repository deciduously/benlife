#![warn(clippy::pedantic)]

use crossbeam::channel;
use eframe::emath::Vec2;
use parking_lot::RwLock;
use std::sync::{
	atomic::{AtomicBool, AtomicU8},
	Arc,
};

mod app;
mod render;
mod rle;
mod universe;

use app::App;
use universe::{Generation, DEFAULT_CELL_SIZE};

/*
- Set up native GUI integration options.
- Instantiate thread-safe universe location.
	- An `Arc<T>` is an atomic reference-counted pointer.  It can be shared safely across threads.
		-`Arc::clone()` creates a new reference to the same heap value, increasing the reference count.
		- Heap memory is freed when the reference count hits zero.
	- `RwLock` is a synchronization primitive which allows multiple concurrent readers or a single writer, not both.
	- Threads will block on calls to `read()` or `write()` until the lock is available.
- Similarly, use an atomic types to coordinate on some shared context.
- Set up channel to communicate between the simulation and UI threads.
	- Uses a multi-producer, multi-consumer channel with a channel size of 0.
		- This channel will block on sends waiting for the buffer to open up.
		- With a buffer size of 0, `send` won't return until `recv` is called.
- Spawn simulation thread.
- Kick off the GUI.
 */
fn main() {
	let native_options = native_options();
	let context = Arc::new(Context::default());
	let (app_sender, ui_receiver) = channel::bounded(0);
	{
		let context = Arc::clone(&context);
		std::thread::spawn(move || {
			let mut app = App::new(ui_receiver, context);
			app.run();
		});
	}
	eframe::run_native(
		"Life of Ben",
		native_options,
		Box::new(|cc| Box::new(render::EguiApp::new(cc, app_sender, context))),
	);
}

/// Data that's shared between threads.
pub(crate) struct Context {
	/// The current universe's cell size.
	cell_size: AtomicU8,
	/// A single generation of the universe, with its generation number.
	shared_gen: RwLock<Generation>,
	/// Whether the simulation is in run mode.
	running: AtomicBool,
	/// The current user speed setting,
	speed: AtomicU8,
}

impl Default for Context {
	fn default() -> Self {
		Self {
			cell_size: AtomicU8::new(DEFAULT_CELL_SIZE),
			shared_gen: RwLock::new(Generation::new()),
			running: AtomicBool::new(false),
			speed: AtomicU8::new(50),
		}
	}
}

fn load_icon() -> eframe::IconData {
	// `include_bytes!()` slurps up the file as a chunk o' bytes in the EXE data section.
	let bytes = include_bytes!("../Life of Dan.ico");
	let icon = image::io::Reader::new(std::io::Cursor::new(bytes))
		.with_guessed_format()
		.expect("Should be infallible")
		.decode()
		.expect("Could not decode image");
	let icon = icon.into_rgba8();
	let width = icon.width();
	let height = icon.height();
	eframe::IconData {
		rgba: icon.into_raw(),
		width,
		height,
	}
}

fn native_options() -> eframe::NativeOptions {
	let icon = load_icon();
	let window_size = Vec2::new(1280.0, 1100.0);
	eframe::NativeOptions {
		decorated: true,
		icon_data: Some(icon),
		initial_window_size: Some(window_size),
		..Default::default()
	}
}
