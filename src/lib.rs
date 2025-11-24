//! This crate provides procedural macros:
//! - `#[derive(AutoGetters)]`
//! - `#[optional]`
//!
//! They're helping for create structures without lots of code
//!
//! <br>
//!
//! # Example
//!
//! ```
//! use fast_struct::AutoGetters;
//!
//! #[derive(AutoGetters)]
//! struct Foo {
//!     bar: i16,
//!     baz: String,
//! }
//! ```
//! 
//! will generate:
//! 
//! ```
//! struct Foo {
//!     bar: i16,
//!     baz: String,
//! }
//!
//! impl Foo {
//!     pub fn bar(&self) -> &i16 {
//!         &self.bar
//!     }
//!
//!     pub fn baz(&self) -> &String {
//!         &self.baz
//!     }
//! }
//! ```

mod auto_getters;
mod optional;
mod builder;

use proc_macro::TokenStream;
use crate::auto_getters::auto_getters_impl;
use crate::builder::builder_impl;
use crate::optional::optional_impl;

/// Automatically generates ***getter methods*** only for **Named Structures**
/// 
/// # Example
///
/// ```
/// use fast_struct::AutoGetters;
///
/// #[derive(AutoGetters)]
/// struct Foo {
///     bar: i16,
///     baz: String,
/// }
/// ```
///
/// will implement these methods for `Foo`:
///
/// ```
/// struct Foo {
///     bar: i16,
///     baz: String,
/// }
///
/// impl Foo {
///     pub fn bar(&self) -> &i16 {
///         &self.bar
///     }
///
///     pub fn baz(&self) -> &String {
///         &self.baz
///     }
/// }
/// ```
#[proc_macro_derive(AutoGetters)]
pub fn auto_getters(input: TokenStream) -> TokenStream {
    auto_getters_impl(input)
}

/// Makes all fields of **Named/Unnamed Structures** optional
///
/// Structure`s fields accept attributes:
/// - `#[optional(except)]` for leave as is
///
/// # Example
///
/// ```
/// use fast_struct::optional;
///
/// #[optional]
/// pub struct Foo {
///     #[except]
///     bar: bool,
///     baz: usize,
///     qux: String,
///     quux: Vec<String>,
/// }
/// ```
///
/// will generate:
///
/// ```
/// pub struct Foo {
///     bar: bool,
///     baz: Option<usize>,
///     qux: Option<String>,
///     quux: Option<Vec<String>>
/// }
/// ```
#[proc_macro_attribute]
pub fn optional(_attr: TokenStream, item: TokenStream) -> TokenStream {
    optional_impl(_attr, item)
}

/// Adds builder for **Named Structures**
///
/// # Example
///
/// ```
/// use fast_struct::Builder;
///
/// #[derive(Builder)]
/// pub struct Foo {
///     bar: bool,
///     baz: String,
///     qux: i16,
/// }
/// ```
///
/// will generate:
///
/// ```
/// pub struct Foo {
///     bar: bool,
///     baz: String,
///     qux: i16,
/// }
///
/// #[derive(Debug)]
/// pub enum FooBuilderError { BarNotFound, BazNotFound, QuxNotFound }
///
/// impl std::fmt::Display for FooBuilderError {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{}", self)
///     }
/// }
///
/// impl std::error::Error for FooBuilderError {}
///
/// impl Foo {
///     pub fn builder() -> FooBuilder {
///         FooBuilder::default()
///     }
/// }
///
/// #[derive(Default, Clone)]
/// pub struct FooBuilder {
///     bar: Option<bool>,
///     baz: Option<String>,
///     qux: Option<i16>,
/// }
///
/// impl FooBuilder {
///     pub fn bar<T: Into<bool>>(&mut self, value: T) -> &mut Self {
///         self.bar = Some(value.into());
///
///         self
///     }
///     pub fn baz<T: Into<String>>(&mut self, value: T) -> &mut Self {
///         self.baz = Some(value.into());
///
///         self
///     }
///     pub fn qux<T: Into<i16>>(&mut self, value: T) -> &mut Self {
///         self.qux = Some(value.into());
///
///         self
///     }
/// }
/// ```
#[proc_macro_derive(Builder)]
pub fn builder(input: TokenStream) -> TokenStream {
    builder_impl(input)
}
