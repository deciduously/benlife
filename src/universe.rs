use rand::Rng;

pub const DEFAULT_CELL_SIZE: u8 = 10;
const DEFAULT_ROWS: usize = 100;
const DEFAULT_COLS: usize = 100;

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
		Self::new(DEFAULT_ROWS, DEFAULT_COLS)
	}
}

/// The `Grid` handles the Game of Life universe.
pub struct Universe {
	/// The state of the universe.
	pub map: Grid,
	/// Internal map for computing the next generation.
	switchmap: Grid,
	/// The number of rows in the grid.
	pub rows: usize,
	/// The number of columns in the grid.
	pub cols: usize,
	/// The size of each individual grid square.
	pub cell_size: u8,
	/// Generation counter.
	pub gen_count: usize,
}

impl Universe {
	/// Instantiate a new `Grid`.
	pub fn new() -> Self {
		let mut universe = Self::default();
		universe.randomize();
		universe
	}

	/// Clear the grid
	pub fn clear(&mut self) {
		self.map = Grid::new(self.rows, self.cols);
		self.switchmap = Grid::new(self.rows, self.cols);
		self.gen_count = 0;
	}

	/// Compute a generation.
	pub fn advance_generation(&mut self) {
		self.gen_count += 1;

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
							&& ((row + r1 >= 0)
								&& (col + c1 >= 0) && (row + r1 < self.rows.try_into().unwrap())
								&& (col + c1 < self.cols.try_into().unwrap()))
							&& self.map.get(
								(row + r1).try_into().unwrap(),
								(col + c1).try_into().unwrap(),
							) {
							n += 1;
						}
					}
				}

				self.switchmap.set(urow, ucol, false);
				let current_state = self.map.get(urow, ucol);
				if (n < 2) && current_state {
					self.switchmap.set(urow, ucol, false);
				} else if ((n == 2) || (n == 3)) && current_state {
					self.switchmap.set(urow, ucol, true);
				} else if (n > 3) && self.map.get(urow, ucol) {
					self.switchmap.set(urow, ucol, false);
				} else if !current_state && n == 3 {
					self.switchmap.set(urow, ucol, true);
				}
			}
		}

		// Copy back to main grid map.
		for (row_idx, row) in self.switchmap.cells.iter().enumerate() {
			for (col_idx, &state) in row.iter().enumerate() {
				self.map.set(row_idx, col_idx, state);
			}
		}
	}

	/// Resize the grid dimensions and re-instantiate.
	pub fn resize(&mut self, columns: usize, rows: usize, cell_size: u8) {
		self.cols = columns;
		self.rows = rows;
		self.cell_size = cell_size;
		self.map = Grid::new(self.rows, self.cols);
		self.switchmap = Grid::new(self.rows, self.cols);
		self.gen_count = 0;
		self.randomize();
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
		for row in &mut self.map.cells {
			for cell in row {
				// Each cell has a 1/3 change to live.
				let rand = rand::thread_rng().gen_range(0..3);
				*cell = rand == 1;
			}
		}
	}
}

impl Default for Universe {
	fn default() -> Self {
		Universe {
			map: Grid::default(),
			switchmap: Grid::default(),
			rows: DEFAULT_ROWS,
			cols: DEFAULT_COLS,
			cell_size: DEFAULT_CELL_SIZE,
			gen_count: 0,
		}
	}
}
