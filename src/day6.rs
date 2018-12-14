use crate::util;

/*
What is the size of the largest area that isn't infinite?

This is really slow, and I don't know why.
*/
pub fn a() {
   let coords = get_coords();
   let mut bounds = (100, 100, 100, 100); // (left, top, right, bottom)
                                          // bounds: 40, 55, 354, 358

   for c in &coords {
      // Check for boundaries
      if c.0 < bounds.0 {
         bounds.0 = c.0
      }
      if c.0 > bounds.2 {
         bounds.2 = c.0
      }
      if c.1 < bounds.1 {
         bounds.1 = c.1
      }
      if c.1 > bounds.3 {
         bounds.3 = c.1
      }
   }
   println!("bounds: {:?}", bounds);

   // let w = bounds.2 - bounds.0;
   // let h = bounds.3 - bounds.1;
   let mut grid = [[9999; 360]; 360];
   for x in bounds.0..=bounds.2 {
      for y in bounds.1..=bounds.3 {
         grid[x][y] = find_closest(&coords, &(x, y));
      }
   }

   // Now we have an array of arrays. It has 9999 in the out-of-bounds squares - ignore them.

   // Count the squares which are closest to each of the 50 points in the input
   println!("6A: ??");
}

fn get_coords() -> Vec<(usize, usize)> {
   let s: String = util::read_file_to_string("input6");
   let lines: Vec<&str> = s.lines().collect();
   let mut coords: Vec<(usize, usize)> = Vec::new();

   for line in lines {
      let pair: Vec<&str> = line.split(", ").collect();
      coords.push((pair[0].parse().unwrap(), pair[1].parse().unwrap()));
   }

   return coords;
}

fn find_closest(coords: &Vec<(usize, usize)>, target: &(usize, usize)) -> usize {
   // STUB
   // Find the closest point in coords to target
   // Return the index of that point in coords
   return 23;
}
