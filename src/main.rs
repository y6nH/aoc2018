extern crate chrono;
extern crate regex;

use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day4;
mod util;

fn main() {
  day1::a();
  day1::b();
  day2::a();
  day2::b();
  day3::a();
  day4::a();
}
