//! Swappable GUI.

use crate::{
	app::{FromUiMsg, ToUiMeg},
	universe::{Generation, DEFAULT_CELL_SIZE},
};
use crossbeam::channel;
use eframe::{
	egui::{self, RichText},
	emath::{Pos2, Vec2},
	epaint::{Color32, RectShape, Rounding},
};
use parking_lot::RwLock;
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};

pub struct EguiApp {
	sender: channel::Sender<FromUiMsg>,
	receiver: channel::Receiver<ToUiMeg>,
	picked_path: Option<String>,
	next_cell_size: u8,
	running: Arc<AtomicBool>,
	shared_gen: Arc<RwLock<Generation>>,
}

impl EguiApp {
	#[must_use]
	pub fn new(
		_cc: &eframe::CreationContext<'_>,
		sender: channel::Sender<FromUiMsg>,
		receiver: channel::Receiver<ToUiMeg>,
		running: Arc<AtomicBool>,
		shared_gen: Arc<RwLock<Generation>>,
	) -> Self {
		// fonts, visuals, cc.storage
		Self {
			sender,
			receiver,
			picked_path: None,
			next_cell_size: DEFAULT_CELL_SIZE,
			running,
			shared_gen,
		}
	}

	/// Render the control panel.
	fn control_panel(&mut self, ui: &mut egui::Ui) {
		ui.heading(
			self.picked_path
				.as_ref()
				.unwrap_or(&"Nothing picked!".to_owned()),
		);
		let generation_count = self.shared_gen.read().gen_count;
		ui.label(format!("Generation count: {generation_count}"));

		ui.group(|ui| {
			ui.label("Cell size");
			let mut size_string = self.next_cell_size.to_string();
			let cell_size_handle = ui.text_edit_singleline(&mut size_string);
			if cell_size_handle.changed() {
				if let Ok(new_cell_size) = size_string.parse() {
					self.next_cell_size = new_cell_size;
				}
			}
			if ui.button("New Grid").clicked() {
				self.sender
					.send(FromUiMsg::NewGrid(self.next_cell_size))
					.unwrap();
			}
		});
		ui.separator();
		if ui.button(self.run_button_text()).clicked() {
			let current = self.running.load(Ordering::Relaxed);
			self.running.swap(!current, Ordering::Relaxed);
		}
		if ui.button("One Generation").clicked() {
			self.sender.send(FromUiMsg::AdvanceOne).unwrap();
		}
		if ui.button("Clear").clicked() {
			self.sender.send(FromUiMsg::Clear).unwrap();
		}
	}

	/// Render the main pain with the Game of Life universe grid.
	fn main_panel(&mut self, ui: &mut egui::Ui) {
		// TODO - we need to resize the cell grid to the right size for the cell size
		// It should stay the same size overall.
		// Paint the grid, one rectangle at a time.
		let mut shapes = Vec::new();
		let cell_size = self.shared_gen.read().cell_size.try_into().unwrap();
		let dimensions = Vec2::splat(cell_size);

		// Compute the UI region's top left coordinate
		let panel_top_left = ui.min_rect().min;
		let x_offset = panel_top_left.x;
		let y_offset = panel_top_left.y;

		for (row_idx, row) in self.shared_gen.read().map.cells.iter().enumerate() {
			for (col_idx, &cell) in row.iter().enumerate() {
				if cell {
					#[allow(clippy::cast_precision_loss)]
					let top_left = Pos2::new(
						row_idx as f32 * cell_size + x_offset,
						col_idx as f32 * cell_size + y_offset,
					);
					let rect = egui::Rect::from_min_size(top_left, dimensions);
					let shape = egui::Shape::Rect(RectShape::filled(
						rect,
						Rounding::none(),
						Color32::BLACK,
					));
					shapes.push(shape);
				}
			}
		}
		let painter = ui.painter();
		painter.extend(shapes);
	}

	/// Render the topbar File/Life/Help menu buttons.
	fn topbar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
		ui.with_layout(egui::Layout::left_to_right(eframe::emath::Align::Center), |ui| {
			ui.menu_button("File", |ui| {
				if ui.button("Open File").clicked() {
					if let Some(path) = rfd::FileDialog::new().pick_file() {
						self.picked_path = Some(path.display().to_string());
					}
				}
				if ui.button("Save File").clicked() {
					if let Some(path) = rfd::FileDialog::new()
						.set_file_name("universe.rle")
						.save_file()
					{
						rfd::MessageDialog::new()
							.set_title("Saved")
							.set_description(&format!("{} saved successfully", path.display()));
					}
				}
				if ui.button("Exit").clicked() {
					self.sender.send(FromUiMsg::Shutdown).unwrap();
					std::process::exit(0);
				}
			});
			ui.menu_button("Life", |ui| {
				if ui.button("Run Generation").clicked() {
					self.sender.send(FromUiMsg::AdvanceOne).unwrap();
				}
				if ui.button(self.run_button_text()).clicked() {
					let current = self.running.load(Ordering::Relaxed);
					self.running.swap(!current, Ordering::Relaxed);
				}
			});
			ui.menu_button("Help", |ui| {
				if ui.button("Release Notes").clicked() {
					// Release Notes
				}
				if ui.button("About").clicked() {
					// About - TODO how do I do this?
					egui::Window::new("about").show(ctx, |ui| {
						ui.heading("About");
					});
				}
			})
		});
	}

	fn run_button_text(&self) -> RichText {
		let (text, color) = if self.running.load(Ordering::Relaxed) {
			("Stop", Color32::RED)
		} else {
			("Run", Color32::GREEN)
		};
		RichText::new(text).color(color).size(20.0)
	}
}

impl eframe::App for EguiApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("topbar_menus").show(ctx, |ui| self.topbar(ctx, ui));
		egui::SidePanel::left("controls").show(ctx, |ui| self.control_panel(ui));
		egui::CentralPanel::default().show(ctx, |ui| self.main_panel(ui));
	
		if let Ok(msg) = self.receiver.try_recv() {
			match msg {
				ToUiMeg::Repaint => {
					ctx.request_repaint();
				}
			}
		}
	}
}
