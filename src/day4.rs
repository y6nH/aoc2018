use crate::util;
use chrono::prelude::*;
use std::cmp;
use std::collections::HashMap;
use time;

/*
Find the guard that has the most minutes asleep. What minute does that guard spend asleep the most?
What is the ID of the guard you chose multiplied by the minute you chose?
*/

pub fn a() {
  let s: String = util::read_file_to_string("input4");
  let mut sorted: Vec<&str> = s.lines().collect();
  let mut current_id: &str = "?";
  let mut timeline: Vec<(&str, &str, &str)> = Vec::new();

  sorted.sort();

  for line in sorted {
    let timestamp = &line[1..17]; // "1518-05-16 23:57"
    let operation = &line[19..24]; // "Guard", "falls" or "wakes"
    if operation == "Guard" {
      current_id = &line[26..30]; // Guard ID, may overflow a bit, but will get the answer
    } else {
      timeline.push((timestamp, &current_id, operation))
    }
  }

  let mut guard_sleep_time: HashMap<&str, chrono::Duration> = HashMap::new();
  let mut longest_sleep: (&str, chrono::Duration) = ("?", time::Duration::zero());
  let mut sleep_by_minute = [0; 60];
  let mut sleep_start: DateTime<Utc> = Utc::now();

  for event in timeline {
    if event.2 == "falls" {
      match Utc.datetime_from_str(event.0, "%Y-%m-%d %H:%M") {
        Ok(d) => sleep_start = d,
        Err(e) => println!("{}: {}", e, event.0),
      }
    } else {
      match Utc.datetime_from_str(event.0, "%Y-%m-%d %H:%M") {
        Ok(d) => {
          let guard_id = event.1;
          let sleep_end: DateTime<Utc> = d;
          let sleep_time = sleep_end - sleep_start;

          for m in sleep_start.minute()..sleep_end.minute() {
            sleep_by_minute[m as usize] += 1;
          }

          // If guard already has sleep time logged, remove, add and reinsert.
          match guard_sleep_time.remove(guard_id) {
            Some(t) => guard_sleep_time.insert(guard_id, t + sleep_time),
            None => guard_sleep_time.insert(guard_id, sleep_time),
          };

          if longest_sleep.1 < sleep_time {
            longest_sleep = (guard_id, sleep_time);
          }
        }
        Err(e) => println!("{}: {}", e, event.0),
      }
    }
  }

  let top_sleeper = guard_sleep_time.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();

  // This is the answer to the wrong question...
  println!("Longest sleep: {:?}", longest_sleep);
  println!("Guard sleeping longest: {:?}", top_sleeper);
}
