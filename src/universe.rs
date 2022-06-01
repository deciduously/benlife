use std::path::Iter;

// TODO this is the stuff inherited from the orig.
const BORDER_OFFSET: u8 = 25;
const DEFAULT_ROWS: usize = 100;
const DEFAULT_COLS: usize = 100;
const GRID_CELLSPACE: u8 = 1;
const LOAD_COL_OFFSET: u8 = 10;
const LOAD_ROW_OFFSET: u8 = 10;
const TESTSHIFT: u8 = 5;
const X_OFFSSET: u8 = 200;
const Y_OFFSET: u8 = 25;

/// The data structure holding the universe state.
struct Grid {
	items: Vec<Vec<bool>>,
}

impl Grid {
	/// Instantiate an empty [`Grid`].
	pub fn new(rows: usize, cols: usize) -> Self {
		let items = vec![vec![false; cols]; rows];
		Self { items }
	}

	/// Retrieve the current value at the given location.
	pub fn get(&self, row: usize, col: usize) -> bool {
		self.items[row][col]
	}

	/// Retreive the total length of this [`Grid`].
	pub fn len(&self) -> usize {
		self.items[0].len() * self.items.len()
	}
}

impl Default for Grid {
	fn default() -> Self {
		Self::new(DEFAULT_ROWS, DEFAULT_COLS)
	}
}

impl IntoIterator for Grid {
	type Item = bool;
	type IntoIter = GridIter;
	fn into_iter(self) -> Self::IntoIter {
		GridIter {
			grid: self,
			col: 0,
			row: 0,
		}
	}
}

struct GridIter {
	grid: Grid,
	col: usize,
	row: usize,
}

impl Iterator for GridIter {
	type Item = bool;
	fn next(&mut self) -> Option<Self::Item> {
		let ret = self.grid.get(self.row, self.col);
		todo!()
	}
}

/// The `Grid` handles the Game of Life universe.
pub struct Universe {
	map: Grid,
	switchmap: Grid,
	/// The number of rows in the grid.
	pub rows: usize,
	/// The number of columns in the grid.
	pub cols: usize,
	/// The size of each individual grid square.
	pub cell_size: usize,
	/// Generation counter.
	pub gen_count: usize,
}

impl Universe {
	/// Instantiate a new `Grid`.
	pub fn new() -> Self {
		Self::default()
	}

	/// Clear the grid
	pub fn clear(&mut self) {
		self.map = Grid::new(self.rows, self.cols);
		self.switchmap = Grid::new(self.rows, self.cols);
	}

	/// Compute a generation.
	pub fn advance_generation(&mut self) {
		self.gen_count += 1;

		// Iteratate over cells
		// switchmap[r][c] = map[r + TESTSHIFT][c + TESTSHIFT]

		// for row in self.map.into_iter() {
		// 	todo!()
		// }
	}

	/// Run infinitely.
	// TODO - I think this is the renderer's job, or at least main
	// pub fn run(&mut self) {
	// 	todo!()
	// }

	/// Resize the grid dimensions and re-instantiate.
	pub fn resize(&mut self, columns: usize, rows: usize, cell_size: usize) {
		self.cols = columns;
		self.rows = rows;
		self.cell_size = cell_size;
		self.map = Grid::new(self.rows, self.cols);
		self.switchmap = Grid::new(self.rows, self.cols);
		self.randomize();
	}

	/// I dont know why we need this either
	fn set_grid(&mut self, row: usize, col: usize) {
		todo!()
	}

	/// Set a point, computing offsets and cellsize.
	// TODO - this method deals with translating mouse coords to grid coords.  Where does it belong?
	pub fn set_point(&self, row: usize, col: usize) {
		todo!()
	}

	/// Toggle the state.
	// TODO what does this do?
	pub fn toggle(&mut self) {
		todo!()
	}

	// I think this will be a method on Renderer.
	// /// Draw the grid
	// pub fn draw(&self) {
	// 	todo!()
	// }

	/// Populate the grid with random live squares.
	fn randomize(&mut self) {
		// for cell in self.iter_mut() {
		// 	todo!()
		// }
		todo!()
	}
}

impl Default for Universe {
	fn default() -> Self {
		Universe {
			map: Grid::default(),
			switchmap: Grid::default(),
			rows: DEFAULT_ROWS,
			cols: DEFAULT_COLS,
			cell_size: 10,
			gen_count: 0,
		}
	}
}
