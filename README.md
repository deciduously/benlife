# benlife

A Rust reimplementation of [`danlife`](https://danlovy.com/life/), for kicks.

## TODO

- [ ] Editing
- [ ] Loading
- [ ] Saving
- [ ] Keyboard Commands
- [ ] Multithreading
- [ ] Pop-up windows

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
