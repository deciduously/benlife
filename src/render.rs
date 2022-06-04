//! Swappable GUI.

use eframe::{
	egui::{self, RichText},
	emath::{Pos2, Vec2},
	epaint::{Color32, RectShape, Rounding},
};

pub struct EguiApp {
	app: crate::app::App,
	picked_path: Option<String>,
}

impl EguiApp {
	#[must_use]
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		// fonts, visuals, cc.storage
		Self {
			app: crate::app::App::new(),
			picked_path: None,
		}
	}

	fn run_button_text(&self) -> RichText {
		let (text, color) = if self.app.running {
			("Stop", Color32::RED)
		} else {
			("Run", Color32::GREEN)
		};
		RichText::new(text).color(color).size(20.0)
	}
}

impl eframe::App for EguiApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("topbar_menus").show(ctx, |ui| {
			ui.with_layout(egui::Layout::left_to_right(), |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("Open File").clicked() {
						if let Some(path) = rfd::FileDialog::new().pick_file() {
							self.picked_path = Some(path.display().to_string());
						}
					}
					if ui.button("Save File").clicked() {
						if let Some(path) = rfd::FileDialog::new().save_file() {
							rfd::MessageDialog::new()
								.set_title("Saved")
								.set_description(&format!("{} saved successfully", path.display()));
						}
					}
				});
				ui.menu_button("Life", |ui| {
					if ui.button("Run Generation").clicked() {
						self.app.universe.advance_generation();
					}
					if ui.button(self.run_button_text()).clicked() {
						self.app.toggle_running();
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
		});
		egui::SidePanel::left("controls").show(ctx, |ui| {
			ui.heading(
				self.picked_path
					.as_ref()
					.unwrap_or(&"Nothing picked!".to_owned()),
			);
			let generation_count = self.app.universe.gen_count;
			ui.label(format!("Generation count: {generation_count}"));

			ui.group(|ui| {
				ui.label("Cell size");
				let mut size_string = self.app.next_cell_size.to_string();
				let cell_size_handle = ui.text_edit_singleline(&mut size_string);
				if cell_size_handle.changed() {
					if let Ok(new_cell_size) = size_string.parse() {
						self.app.next_cell_size = new_cell_size;
					}
				}
				if ui.button("New Grid").clicked() {
					self.app.new_grid();
				}
			});
			ui.separator();
			if ui.button(self.run_button_text()).clicked() {
				self.app.toggle_running();
			}
			if ui.button("One Generation").clicked() {
				self.app.universe.advance_generation();
			}
			if ui.button("Clear").clicked() {
				self.app.universe.clear();
			}
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			// TODO - we need to resize the cell grid to the right size for the cell size
			// It should stay the same size overall.
			// Paint the grid, one rectangle at a time.
			let mut shapes = Vec::new();
			let cell_size = self.app.universe.cell_size.try_into().unwrap();
			let dimensions = Vec2::splat(cell_size);

			// Compute the UI region's top left coordinate
			let panel_top_left = ui.min_rect().min;
			let x_offset = panel_top_left.x;
			let y_offset = panel_top_left.y;

			for (row_idx, row) in self.app.universe.map.cells.iter().enumerate() {
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
		});
		if self.app.running {
			self.app.universe.advance_generation();
		}
	}
}
