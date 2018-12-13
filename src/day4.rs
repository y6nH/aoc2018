use crate::util;
use chrono::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use time;

/*
This one is a mess. It could be rewritten to be much shorter and more efficient.

A: Find the guard that has the most minutes asleep. What minute does that guard spend asleep the most?
What is the ID of the guard you chose multiplied by the minute you chose?

B: Of all guards, which guard is most frequently asleep on the same minute?
What is the ID of the guard you chose multiplied by the minute you chose?
*/

pub fn a() {
  let s: String = util::read_file_to_string("input4");
  let mut sorted: Vec<&str> = s.lines().collect();
  let mut current_id: &str = "?";
  let mut timeline: Vec<(&str, &str, &str)> = Vec::new();
  let mut guard_ids: HashSet<&str> = HashSet::new();

  sorted.sort();

  for line in sorted {
    let timestamp = &line[1..17]; // "1518-05-16 23:57"
    let operation = &line[19..24]; // "Guard", "falls" or "wakes"
    if operation == "Guard" {
      current_id = &line[26..30]; // Guard ID, may overflow a bit, but will get the answer
      let num_id: Vec<&str> = current_id.split_whitespace().collect();
      guard_ids.insert(num_id[0]);
    } else {
      timeline.push((timestamp, &current_id, operation))
    }
  }

  let mut guard_sleep_time: HashMap<&str, chrono::Duration> = HashMap::new();
  let mut longest_sleep: (&str, chrono::Duration) = ("?", time::Duration::zero());
  let mut sleep_by_minute = [0; 60];
  let mut sleep_start: DateTime<Utc> = Utc::now();
  let mut sleep_by_guard_by_minute: Vec<(&str, [usize; 60])> = Vec::new();

  for event in &timeline {
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

  for id in guard_ids {
    let mut sleep_log = [0; 60];
    for event in timeline.iter().filter(|a| &a.1 == &id) {
      if event.2 == "falls" {
        match Utc.datetime_from_str(event.0, "%Y-%m-%d %H:%M") {
          Ok(d) => sleep_start = d,
          Err(e) => println!("{}: {}", e, event.0),
        }
      } else {
        match Utc.datetime_from_str(event.0, "%Y-%m-%d %H:%M") {
          Ok(d) => {
            let sleep_end: DateTime<Utc> = d;
            for m in sleep_start.minute()..sleep_end.minute() {
              sleep_log[m as usize] += 1;
            }
          }
          Err(e) => println!("{}: {}", e, event.0),
        }
      }
    }

    sleep_by_guard_by_minute.push((&id, sleep_log));
  }

  let top_guard_and_minute = sleep_by_guard_by_minute
    .iter()
    .max_by(|&x, &y| {
      x.1
        .iter()
        .max_by(|&v, &w| v.cmp(w))
        .cmp(&y.1.iter().max_by(|&v, &w| v.cmp(w)))
    })
    .unwrap();

  let tgm_guard : usize = top_guard_and_minute.0.parse().unwrap();
  let tgm_times = top_guard_and_minute.1.iter().max().unwrap();
  let tgm_minute = top_guard_and_minute.1.iter().position(|m| m == tgm_times).unwrap();

  let top_sleeper = guard_sleep_time.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
  let top_sleeper_id: usize = top_sleeper.0.parse().unwrap();

  // This is a long way round... because I answered the wrong question at first
  for event in timeline.iter().filter(|a| &a.1 == top_sleeper.0) {
    if event.2 == "falls" {
      match Utc.datetime_from_str(event.0, "%Y-%m-%d %H:%M") {
        Ok(d) => sleep_start = d,
        Err(e) => println!("{}: {}", e, event.0),
      }
    } else {
      match Utc.datetime_from_str(event.0, "%Y-%m-%d %H:%M") {
        Ok(d) => {
          let sleep_end: DateTime<Utc> = d;

          for m in sleep_start.minute()..sleep_end.minute() {
            sleep_by_minute[m as usize] += 1;
          }
        }
        Err(e) => println!("{}: {}", e, event.0),
      }
    }
  }

  let most_sleep = sleep_by_minute.iter().max().unwrap();

  // sleepiest minute of the sleepiest guard
  let sleepiest_minute = sleep_by_minute
    .iter()
    .position(|&x| &x == most_sleep)
    .unwrap();
  // Some answers to the wrong questions...
  println!("Longest sleep: {:?}", longest_sleep);
  println!("Guard sleeping longest: {:?}", top_sleeper);
  println!("Sleepiest minute: {}", sleepiest_minute);
  println!("4A: {}", sleepiest_minute * top_sleeper_id);
  println!(
    "4B: {} * {} = {}",
    tgm_guard, tgm_minute, tgm_guard * tgm_minute
  );
}
