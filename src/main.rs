#![warn(clippy::pedantic)]

use eframe::emath::Vec2;

mod app;
mod render;
mod universe;

fn main() {
	let window_size = Vec2::new(1280.0, 1100.0);
	// TODO - IconData
	let native_options = eframe::NativeOptions {
		decorated: true,
		initial_window_size: Some(window_size),
		..Default::default()
	};
	eframe::run_native(
		"Life of Ben",
		native_options,
		Box::new(|cc| Box::new(render::EguiApp::new(cc))),
	)
}
