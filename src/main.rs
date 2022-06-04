#![warn(clippy::pedantic)]

use eframe::emath::Vec2;

mod app;
mod render;
mod universe;

fn main() {
	let icon = load_icon();
	let window_size = Vec2::new(1280.0, 1100.0);
	let native_options = eframe::NativeOptions {
		decorated: true,
		icon_data: Some(icon),
		initial_window_size: Some(window_size),
		..Default::default()
	};
	eframe::run_native(
		"Life of Ben",
		native_options,
		Box::new(|cc| Box::new(render::EguiApp::new(cc))),
	)
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
