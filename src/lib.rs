/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![cfg_attr(feature = "plugins", feature(plugin))]
#![cfg_attr(feature = "plugins", feature(custom_derive))]
#![cfg_attr(feature = "plugins", plugin(serde_macros))]

extern crate euclid;
extern crate rustc_serialize;
#[cfg(feature = "plugins")]
extern crate serde;

mod app_unit;

pub use app_unit::{Au, MIN_AU, MAX_AU, AU_PER_PX};
