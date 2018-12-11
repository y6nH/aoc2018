use chrono;
use crate::util;

pub fn a() {
  let s: String = util::read_file_to_string("input4");
  let mut sorted = s.lines();
  let mut current_id : &str = "?";
  let mut timeline : Vec<(&str, &str, &str)> = Vec::new();

  for line in sorted {
    let timestamp = &line[1..17]; // "1518-05-16 23:57"
    let operation = &line[19..24]; // "Guard", "falls" or "wakes"
    if operation == "Guard"{
      current_id = &line[26..30]; // Guard ID, may overflow a bit, but will get the answer
    } else {
      timeline.push((timestamp, &current_id, operation))
    }
  }

}
