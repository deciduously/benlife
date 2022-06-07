# benlife

A Rust reimplementation of [`danlife`](https://danlovy.com/life/), for kicks.

## TODO

- [ ] Editing universe grapically
- [ ] Loading [RLE](https://conwaylife.com/wiki/Run_Length_Encoded) files
- [ ] Saving RLE files
- [ ] Keyboard Commands
- [ ] Multithreading
- [ ] Pop-up windows
- [ ] Support alternate patterns like HighLife.
- [ ] Support [apgcode](https://conwaylife.com/wiki/Apgcode)
- [ ] Handle panics.

## Comparison

### Rust Pros

- Exhaustive enums, `match` over `switch` (no ``break`!).
- Cross-platform.

### Rust Cons

### C++ Pros

- Code size. Rust statically links all deps, I also currently bundle a whole UI framework.

### C++ Cons

- Manual memory management (`new`/`delete`/destructors)
- Preprocessor macros
