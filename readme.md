# derive_index

proc_macro library to derive indexing into Vectors with newtypes.

## Usage
Given a newtype like this:
```rust
struct NewType(usize);  // define a struct with inner type that can index something (e.g. Vec<f64>)
```

using derive_index one can derive indexing into Vec<T> and Vec<Option<T>>
```rust
use derive_index::Index;
#[derive(Index)]
#[index_type(f64)]  // index into Vec<f64> and Vec<Option<f64>>
struct NewType(usize);
```
This will expand into
```rust
impl std::ops::Index<NewType> for Vec<f64> {
    type Output = f64;
    fn index(&self, index: NewType) -> &Self::Output {
        self.index(index.0)
    }
}
```
and corresponding implementations for `IndexMut` and `Vec<Option<f64>>`.
