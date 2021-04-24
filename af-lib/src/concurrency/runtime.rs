use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::AcqRel;

use super::{scope, thread};
use crate::log;
use crate::prelude::*;

pub unsafe fn run<O, F>(module_path: &'static str, future: F) -> !
where
  O: scope::IntoOutput + 'static,
  F: Future<Output = O> + 'static,
{
  static ONCE: AtomicBool = AtomicBool::new(false);

  if ONCE.swap(true, AcqRel) {
    panic!("runtime already started");
  }

  let result = thread::run(future);

  if let Err(err) = &result {
    error!(target: module_path, "Main thread {}", fmt::indent("", "  ", err));
  }

  async_io::block_on(log::flush());

  match result {
    Ok(_) => exit(0),
    Err(_) => exit(1),
  }
}
