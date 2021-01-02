// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Initializes `af_runtime::logger`.
#[macro_export]
macro_rules! logger_init {
  () => {
    __af_core::runtime::logger::init();

    __af_core::runtime::logger::set_level_of(
      option_env!("CARGO_BIN_NAME").unwrap_or(env!("CARGO_PKG_NAME")).replace("-", "_"),
      match cfg!(debug_assertions) {
        true => __af_core::log::Debug,
        false => __af_core::log::Info,
      },
    );
  };
}
