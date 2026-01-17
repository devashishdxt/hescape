//! A fast and lightweight HTML escape/unescape library for Rust.
//!
//! This crate provides functions to escape and unescape HTML special characters, which is essential for preventing
//! XSS (Cross-Site Scripting) attacks and correctly rendering user-provided content in HTML documents.
//!
//! ## Escaping
//!
//! The [`escape`] function converts the following characters to their HTML entity equivalents:
//!
//! | Character | Entity     |
//! |-----------|------------|
//! | `&`       | `&amp;`    |
//! | `<`       | `&lt;`     |
//! | `>`       | `&gt;`     |
//! | `"`       | `&quot;`   |
//! | `'`       | `&#39;`    |
//!
//! ### Example
//!
//! ```rust
//! use hescape::escape;
//!
//! let input = "<script>alert(\"xss\")</script>";
//! let escaped = escape(input);
//! assert_eq!(escaped, "&lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;");
//! ```
//!
//! ## Unescaping
//!
//! The [`unescape`] function converts HTML entities back to their original characters.
//! It supports:
//!
//! - **Named references**: `&amp;`, `&lt;`, `&gt;`, `&quot;`, `&apos;`, and [many more](https://html.spec.whatwg.org/multipage/named-characters.html).
//! - **Decimal numeric references**: `&#39;`, `&#60;`, etc.
//! - **Hexadecimal numeric references**: `&#x27;`, `&#x3C;`, etc.
//!
//! ### Example
//!
//! ```rust
//! use hescape::unescape;
//!
//! let input = "&lt;div&gt;Hello &amp; welcome!&lt;/div&gt;";
//! let unescaped = unescape(input);
//! assert_eq!(unescaped, "<div>Hello & welcome!</div>");
//! ```
//!
//! ## Writing to a buffer
//!
//! For performance-sensitive applications, you can use [`escape_to`] and [`unescape_to`] to write directly to any
//! type implementing [`core::fmt::Write`]:
//!
//! ```rust
//! use hescape::escape_to;
//!
//! let mut buffer = String::new();
//! escape_to(&mut buffer, "Hello <world>").unwrap();
//! assert_eq!(buffer, "Hello &lt;world&gt;");
//! ```
mod escape;
mod unescape;

pub use self::{
    escape::{escape, escape_to},
    unescape::{unescape, unescape_to},
};
