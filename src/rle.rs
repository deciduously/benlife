//! This module handles serializeing and deserializing universes from the RLE file format.
//!
//! <https://conwaylife.com/wiki/Run_Length_Encoded>

use std::{
	io::{self, BufRead, BufReader, BufWriter, Read, Write},
	path::Path,
};

use crate::universe::Universe;

/// Deserialize a [`Universe`] from a file.
pub fn from_file(path: impl AsRef<Path>) -> io::Result<Universe> {
	let file = std::fs::File::open(&path)?;
	let reader = BufReader::new(file);
	let lines = reader.lines();
	// Read any # lines

	// Read header line

	// Read grid
	todo!()
}

/// Serialize a [`Universe`] to a writer.
pub fn to_writer(universe: &Universe, writer: &mut impl io::Write) -> io::Result<()> {
	// Write metadata.
	if let Some(name) = &universe.metadata.name {
		writeln!(writer, "#N {name}")?;
	}
	if let Some(author) = &universe.metadata.author {
		writeln!(writer, "#O {author}")?;
	}
	for comment in &universe.metadata.comments {
		writeln!(writer, "#C {comment}")?;
	}
	// Write header.
	let x = universe.rows;
	let y = universe.cols;
	let ruleset = universe.metadata.ruleset.to_string();
	writeln!(writer, "x = {x}, y = {y}, rule = {ruleset}")?;
	// Write grid.
	for row in &universe.map.cells {
		for &col in row {
			let c = if col { 'b' } else { 'o' };
			write!(writer, "{c}")?;
		}
		write!(writer, "$")?;
	}
	writeln!(writer)?;
	Ok(())
}

pub fn to_file(universe: &Universe, path: impl AsRef<Path>) -> io::Result<()> {
	let f = std::fs::File::open(&path)?;
	let mut writer = BufWriter::new(f);
	to_writer(universe, &mut writer)?;
	Ok(())
}

#[cfg(test)]
mod test {
	use super::*;
}
