//! Procedural macro for defining global constructor/destructor functions.
//!
//! This provides module initialization/teardown functions for Rust (like
//! `__attribute__((constructor))` in C/C++) for Linux, OSX, and Windows via
//! the `#[ctor]` and `#[dtor]` macros.
//!
//! This library works and is regularly tested on Linux, OSX and Windows, with both `+crt-static` and `-crt-static`.
//! Other platforms are supported but not tested as part of the automatic builds. This library will also work as expected in both
//! `bin` and `cdylib` outputs, ie: the `ctor` and `dtor` will run at executable or library
//! startup/shutdown respectively.
//!
//! This library currently requires Rust > `1.31.0` at a minimum for the
//! procedural macro support.

#![recursion_limit = "256"]

#[doc(hidden)]
#[allow(unused)]
pub mod __support {
    pub use crate::__ctor_entry as ctor_entry;
    pub use crate::__ctor_link_section as ctor_link_section;
    pub use crate::__ctor_link_section_attr as ctor_link_section_attr;
    #[doc(hidden)]
    pub use crate::__ctor_parse as ctor_parse;
    pub use crate::__dtor_entry as dtor_entry;
    pub use crate::__if_has_feature as if_has_feature;
    pub use crate::__if_unsafe as if_unsafe;
    pub use crate::__unify_features as unify_features;
}

mod macros;

// Code note:

// You might wonder why we don't use `__attribute__((destructor))`/etc for
// dtor. Unfortunately mingw doesn't appear to properly support section-based
// hooks for shutdown, ie:

// https://github.com/Alexpux/mingw-w64/blob/d0d7f784833bbb0b2d279310ddc6afb52fe47a46/mingw-w64-crt/crt/crtdll.c

// In addition, OSX has removed support for section-based shutdown hooks after
// warning about it for a number of years:

// https://reviews.llvm.org/D45578

/// Marks a function or static variable as a library/executable constructor.
/// This uses OS-specific linker sections to call a specific function at
/// load time.
///
/// Multiple startup functions/statics are supported, but the invocation order is not
/// guaranteed.
///
/// # Examples
///
/// Print a startup message (using `libc_print` for safety):
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # extern crate ctor;
/// # use ctor::*;
/// use libc_print::std_name::println;
///
/// #[ctor]
/// fn foo() {
///   println!("Hello, world!");
/// }
///
/// # fn main() {
/// println!("main()");
/// # }
/// ```
///
/// Make changes to `static` variables:
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # extern crate ctor;
/// # mod test {
/// # use ctor::*;
/// # use std::sync::atomic::{AtomicBool, Ordering};
/// static INITED: AtomicBool = AtomicBool::new(false);
///
/// #[ctor]
/// fn foo() {
///   INITED.store(true, Ordering::SeqCst);
/// }
/// # }
/// ```
///
/// Initialize a `HashMap` at startup time:
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # extern crate ctor;
/// # mod test {
/// # use std::collections::HashMap;
/// # use ctor::*;
/// #[ctor]
/// pub static STATIC_CTOR: HashMap<u32, String> = unsafe {
///   let mut m = HashMap::new();
///   for i in 0..100 {
///     m.insert(i, format!("x*100={}", i*100));
///   }
///   m
/// };
/// # }
/// # pub fn main() {
/// #   assert_eq!(test::STATIC_CTOR.len(), 100);
/// #   assert_eq!(test::STATIC_CTOR[&20], "x*100=2000");
/// # }
/// ```
///
/// # Details
///
/// The `#[ctor]` macro makes use of linker sections to ensure that a
/// function is run at startup time.
///
/// The above example translates into the following Rust code (approximately):
///
///```rust
/// #[used]
/// #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".init_array")]
/// #[cfg_attr(target_os = "freebsd", link_section = ".init_array")]
/// #[cfg_attr(target_os = "netbsd", link_section = ".init_array")]
/// #[cfg_attr(target_os = "openbsd", link_section = ".init_array")]
/// #[cfg_attr(target_os = "illumos", link_section = ".init_array")]
/// #[cfg_attr(target_vendor = "apple", link_section = "__DATA,__mod_init_func")]
/// #[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
/// static FOO: extern fn() = {
///   #[cfg_attr(any(target_os = "linux", target_os = "android"), link_section = ".text.startup")]
///   extern fn foo() { /* ... */ };
///   foo
/// };
/// ```
pub use ctor_proc_macro::ctor;

/// Marks a function as a library/executable destructor. This uses OS-specific
/// linker sections to call a specific function at termination time.
///
/// Multiple shutdown functions are supported, but the invocation order is not
/// guaranteed.
///
/// `sys_common::at_exit` is usually a better solution for shutdown handling, as
/// it allows you to use `stdout` in your handlers.
///
/// ```rust
/// # #![cfg_attr(feature="used_linker", feature(used_with_arg))]
/// # extern crate ctor;
/// # use ctor::*;
/// # fn main() {}
///
/// #[dtor]
/// fn shutdown() {
///   /* ... */
/// }
/// ```
pub use ctor_proc_macro::dtor;
