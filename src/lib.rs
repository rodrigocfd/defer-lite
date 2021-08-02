//! A Rust `no_std` implementation of
//! [Go's `defer` statement](https://tour.golang.org/flowcontrol/12) as a macro,
//! which executes a block of code when the surrounding scope ends.
//!
//! # Usage
//!
//! Add the dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! godefer = "1.0.0"
//! ```

#![no_std]

#[doc(hidden)]
pub struct Defer<F: FnOnce()> { // internal struct used by defer! macro
	func: Option<F>,
}

impl<F: FnOnce()> Defer<F> {
	pub fn new(func: F) -> Self {
		Self { func: Some(func) }
	}
}

impl<F: FnOnce()> Drop for Defer<F> {
	fn drop(&mut self) {
		self.func.take().map(|f| f());
	}
}

/// Executes a block of code when the surrounding scope ends.
///
/// # Examples
///
/// Simplest example:
///
/// ```rust
/// use godefer::defer;
///
/// fn main() {
///     defer! { println!("Second"); }
///     println!("First");
/// }
/// ```
///
/// Multiple statements:
///
/// ```rust
/// use godefer::defer;
///
/// fn main() {
///     defer! {
///         println!("Second");
///         println!("Third");
///     }
///     println!("First");
/// }
/// ```
///
/// In Go, the `defer` code runs when the function exits. In this Rust
/// implementation, the code runs when the surrounding scope ends – this makes
/// it possible to use `defer` inside loops:
///
/// ```rust
/// use godefer::defer;
///
/// fn main() {
///     defer! { println!("End"); }
///     println!("Before");
///
///     for i in 0..2 {
///         defer! { println!("Defer {}", i); }
///         println!("Loop {}", i);
///     }
///
///     println!("After");
/// }
/// ```
#[macro_export]
macro_rules! defer {
	( $($tt:tt)* ) => {
		let _deferred = godefer::Defer::new(|| { $($tt)* });
	};
}
