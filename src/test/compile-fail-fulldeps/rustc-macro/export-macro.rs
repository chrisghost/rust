// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern: cannot export macro_rules! macros from a `rustc-macro` crate

#![crate_type = "rustc-macro"]
#![feature(rustc_macro)]

#[macro_export]
macro_rules! foo {
    ($e:expr) => ($e)
}
