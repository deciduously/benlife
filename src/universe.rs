use crate::Context;
use rand::Rng;
use std::sync::Arc;

pub const DEFAULT_CELL_SIZE: u8 = 10;
pub const DEFAULT_CELLS_PER_ROW: usize = 100;
pub const DEFAULT_CELLS_PER_COL: usize = 100;

// TODO this is the stuff inherited from the orig.
// const BORDER_OFFSET: u8 = 25;
// const GRID_CELLSPACE: u8 = 1;
// const LOAD_COL_OFFSET: u8 = 10;
// const LOAD_ROW_OFFSET: u8 = 10;
// const TESTSHIFT: u8 = 5;
// const X_OFFSSET: u8 = 200;
// const Y_OFFSET: u8 = 25;

/// The data structure holding the universe state.
pub struct Grid {
	pub cells: Vec<Vec<bool>>,
}

impl Grid {
	/// Instantiate an empty [`Grid`].
	pub fn new(rows: usize, cols: usize) -> Self {
		let cells = vec![vec![false; cols]; rows];
		Self { cells }
	}

	/// Reset this grid to default.
	pub fn clear(&mut self) {
		for row in &mut self.cells {
			for cell in row {
				*cell = false;
			}
		}
	}

	/// Retrieve the value of the given cell.
	pub fn get(&self, row: usize, col: usize) -> bool {
		self.cells[row][col]
	}

	/// Set the value at the given cell.
	pub fn set(&mut self, row: usize, col: usize, val: bool) {
		self.cells[row][col] = val;
	}
}

impl Default for Grid {
	fn default() -> Self {
		Self::new(DEFAULT_CELLS_PER_ROW, DEFAULT_CELLS_PER_COL)
	}
}

/// The `Grid` handles the Game of Life universe.
pub(crate) struct Universe {
	/// Pointer to state of the universe.
	pub context: Arc<Context>,
	/// Internal map for computing the next generation.
	switchmap: Grid,
	/// The number of rows in the grid.
	pub rows: usize,
	/// The number of columns in the grid.
	pub cols: usize,
	/// Various metadata about the universe.
	pub metadata: Metadata,
}

impl Universe {
	/// Instantiate a new `Grid`.
	pub fn new(context: Arc<Context>) -> Self {
		Self::with_size(context, DEFAULT_CELL_SIZE)
	}

	/// Instantiate a `Grid` with a specific size.
	pub fn with_size(context: Arc<Context>, cell_size: u8) -> Self {
		let (rows, cols) = get_new_dims(cell_size);
		let mut universe = Universe {
			context,
			rows,
			cols,
			metadata: Metadata::default(),
			switchmap: Grid::new(rows, cols),
		};
		universe.context.shared_gen.write().reset(rows, cols);
		universe.randomize();
		universe
	}

	/// Clear the grid
	pub fn clear(&mut self) {
		self.context.shared_gen.write().map.clear();
		self.switchmap.clear();
	}

	/// Compute a generation.
	pub fn advance_generation(&mut self) {
		dbg!("advance");
		self.context.shared_gen.write().gen_count += 1;
		dbg!("reading");
		let read_lock = self.context.shared_gen.read();

		// Iterate over cells
		#[allow(clippy::cast_possible_wrap)]
		for row in 0..self.rows as isize {
			let urow = row.try_into().unwrap();
			#[allow(clippy::cast_possible_wrap)]
			for col in 0..self.cols as isize {
				let ucol = col.try_into().unwrap();
				// Determine number of neighbors.
				let mut n = 0;
				for r1 in -1..2isize {
					for c1 in -1..2isize {
						if !(r1 == 0 && c1 == 0)
							&& (row + r1 >= 0) && (col + c1 >= 0)
							&& (row + r1 < self.rows.try_into().unwrap())
							&& (col + c1 < self.cols.try_into().unwrap())
						{
							dbg!(row);
							dbg!(r1);
							let state = read_lock.map.get(
								(row + r1).try_into().unwrap(),
								(col + c1).try_into().unwrap(),
							);
							if state {
								n += 1;
							}
						}
					}
				}

				self.switchmap.set(urow, ucol, false);
				let current_state = read_lock.map.get(urow, ucol);
				if (n < 2) && current_state {
					self.switchmap.set(urow, ucol, false);
				} else if ((n == 2) || (n == 3)) && current_state {
					self.switchmap.set(urow, ucol, true);
				} else if (n > 3) && read_lock.map.get(urow, ucol) {
					self.switchmap.set(urow, ucol, false);
				} else if !current_state && n == 3 {
					self.switchmap.set(urow, ucol, true);
				}
			}
		}
		drop(read_lock);

		// Copy back to main grid map.
		let mut write_lock = self.context.shared_gen.write();
		for (row_idx, row) in self.switchmap.cells.iter().enumerate() {
			for (col_idx, &state) in row.iter().enumerate() {
				write_lock.map.set(row_idx, col_idx, state);
			}
		}
	}

	/// Resize the grid dimensions and re-instantiate.
	pub fn reset(&mut self, cell_size: u8) {
		*self = Self::with_size(Arc::clone(&self.context), cell_size);
	}

	// /// I dont know why we need this either
	// fn set_grid(&mut self, row: usize, col: usize) {
	// 	todo!()
	// }

	// /// Set a point, computing offsets and cellsize.
	// // TODO - this method deals with translating mouse coords to grid coords.  Where does it belong?
	// pub fn set_point(&self, row: usize, col: usize) {
	// 	todo!()
	// }

	// /// Toggle the state.
	// // TODO what does this do?
	// pub fn toggle(&mut self) {
	// 	todo!()
	// }

	/// Populate the grid with random live squares.
	fn randomize(&mut self) {
		dbg!("randomize");
		for row in &mut self.context.shared_gen.write().map.cells {
			for cell in row {
				// Each cell has a 1/3 change to live.
				let rand = rand::thread_rng().gen_range(0..3);
				*cell = rand == 1;
			}
		}
		dbg!("done");
	}
}

/// Calculate the correct dimensions for the new given cell size.
///
/// Returns (rows, cols)
const fn get_new_dims(new_cell_size: u8) -> (usize, usize) {
	let new_size = new_cell_size as usize;
	let target_row = DEFAULT_CELLS_PER_ROW * DEFAULT_CELL_SIZE as usize;
	let target_col = DEFAULT_CELLS_PER_COL * DEFAULT_CELL_SIZE as usize;
	let row_x = target_row / new_size;
	let col_x = target_col / new_size;
	let new_rows = row_x * DEFAULT_CELLS_PER_ROW;
	let new_cols = col_x * DEFAULT_CELLS_PER_COL;
	(new_rows, new_cols)
}

/// A single frame of the universe and the current generation count.
#[derive(Default)]
pub struct Generation {
	pub map: Grid,
	pub gen_count: usize,
}

impl Generation {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}

	fn reset(&mut self, rows: usize, cols: usize) {
		self.map = Grid::new(rows, cols);
		self.gen_count = 0;
	}
}

#[derive(Default)]
pub struct Metadata {
	pub author: Option<String>,
	pub name: Option<String>,
	pub comments: Vec<String>,
	pub ruleset: Ruleset,
}

/// The pattern to use to compute generations.
#[derive(Debug)]
pub enum Ruleset {
	Life,
	// HighLife,
}

impl Default for Ruleset {
	fn default() -> Self {
		Ruleset::Life
	}
}

impl std::fmt::Display for Ruleset {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Ruleset::Life => "B3/S23",
			// Ruleset::HighLife => "B36/S23",
		};
		write!(f, "{s}")
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_get_new_dims() {
		assert_eq!(get_new_dims(10), (10000, 10000));
	}
}
