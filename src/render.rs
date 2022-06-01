//! Swappable GUI.

use crate::app::App;

trait Renderer {
	/// Render the application.
	fn draw(&self, app: &App);
}
