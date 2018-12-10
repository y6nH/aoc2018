use chrono;
use util;

pub fn a() {
  let s = util::read_file_to_string("input4");
  let mut split = s.split('\n');

  // split.sort(); MAKE THIS WORK

  'next_line: for line in split {
    let timestamp = &line[1..17]; // "1518-05-16 23:57"
    let operation = &line[19..24]; // "Guard", "falls" or "wakes"
  }

  //let mut entries: Vec<(DateTime)> = Vec::new();

  //.parse::<DateTime<Utc>>()
  // [1             16] 19     26
  // [1518-05-16 23:57] Guard #1619 begins shift
  // [1518-11-17 00:13] falls asleep
  // [1518-04-21 00:37] wakes up
}
