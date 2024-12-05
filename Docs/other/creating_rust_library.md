Creating a library in Rust is straightforward, thanks to Rust's modular structure and cargo, its package manager. Here’s a step-by-step guide:

### 1. Set Up the Library Project

1. **Create a new library**: You can create a new library using Cargo with the `--lib` flag:
   ```bash
   cargo new my_library --lib
   ```

   This command will create a folder named `my_library` with the following structure:
   ```
   my_library/
   ├── Cargo.toml
   └── src
       └── lib.rs
   ```

2. **Edit `Cargo.toml`**: This file contains metadata about your library (e.g., name, version, dependencies). It’s automatically configured but can be modified if you want to add dependencies or tweak settings.

### 2. Implement Library Code

1. **Write Code in `lib.rs`**:
   
   Open `src/lib.rs`, which will contain the core code of your library. Here’s a basic example of a function within the library:

   ```rust
   // src/lib.rs
   pub fn greet(name: &str) -> String {
       format!("Hello, {}!", name)
   }
   ```

   Note:
   - Functions or structs that should be accessible from other crates (public) must be declared with `pub`.
   - Everything else is private by default and only accessible within the library.

2. **Add More Modules (Optional)**:
   
   If your library is large, you may want to break it into separate modules. You can do this by creating additional files in the `src` directory.

   ```rust
   // src/lib.rs
   pub mod greetings;

   // src/greetings.rs
   pub fn hello() -> String {
       "Hello from the greetings module!".to_string()
   }
   ```

### 3. Write Documentation (Optional but Recommended)

Add comments to your code to make documentation easier. Rust uses triple slashes `///` for doc comments, which generate documentation.

```rust
/// Greets a user by their name.
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person to greet.
///
/// # Example
///
/// ```
/// let greeting = my_library::greet("Alice");
/// assert_eq!(greeting, "Hello, Alice!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### 4. Testing the Library

You can write tests in the `src/lib.rs` file or in a separate `tests` directory.

```rust
// src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }
}
```

Run the tests with:
```bash
cargo test
```

### 5. Publishing the Library (Optional)

1. **Create an Account on [crates.io](https://crates.io/)** if you haven’t already.
2. **Login to crates.io**:
   ```bash
   cargo login <YOUR_API_KEY>
   ```
3. **Publish the Library**:
   ```bash
   cargo publish
   ```

Now your library is live on `crates.io` and can be added to other Rust projects as a dependency.