#![warn(clippy::pedantic)]

use eframe::emath::Vec2;
use crossbeam::channel;
use parking_lot::RwLock;
use std::sync::{atomic::AtomicBool, Arc};

mod app;
mod render;
mod rle;
mod universe;

use app::App;
use universe::Generation;

fn main() {
	// Set up native GUI integration options.
	let native_options = native_options();

	// Instantiate thread-safe universe location.
	// An `Arc<T>` is an atomic reference-counted pointer.  It can be shared safely across threads.
	// `Arc::clone()` creates a new reference to the same heap value, increasing the reference count.
	// Heap memory is freed when the reference count hits zero.
	// `RwLock` is a synchronization primitive which allows multiple concurrent readers or a single writer, not both.
	// Threads will block on calls to `read()` or `write()` until the lock is available.
	let shared_gen = Arc::new(RwLock::new(Generation::new()));

	// Similarly, use an AtomicBool to coordinate on whether the app is in run mode.
	let running = Arc::new(AtomicBool::new(false));

	// Set up channel to communicate between the simulation and UI threads.
	// Uses a multi-producer, multi-consumer channel with a channel size of 0.
	// This channel will block on sends waiting for the buffer to open up.
	// With a buffer size of 0, `send` won't return until `recv` is called.
	// The app will only wait 1ms per frame before giving up and checking again next frame.
	let (app_sender, ui_receiver) = channel::bounded(0);

	// Spawn simulation thread.
	{
		// Grab scope-local handles to the shared data for the thread.
		let shared_gen = Arc::clone(&shared_gen);
		let running = Arc::clone(&running);
		std::thread::spawn(move || {
			let mut app = App::new(ui_receiver, running, shared_gen);
			app.run();
		});
	}

	// Kick off the GUI.
	eframe::run_native(
		"Life of Ben",
		native_options,
		Box::new(|cc| Box::new(render::EguiApp::new(cc, app_sender, running, shared_gen))),
	);
}

fn load_icon() -> eframe::IconData {
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
