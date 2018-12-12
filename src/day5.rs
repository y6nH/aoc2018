use crate::util;

pub fn a() {
  let s: String = util::read_file_to_string("input4");
  let bytes = s.as_bytes();
}

fn chomp_bytes(bytes: &[u8]) -> &[u8] {
  let l = bytes.len();

  let mut result: &[u8] = &[];

  for i in 1..l {
    let diff = bytes[i].wrapping_sub(bytes[i - 1]);
    match diff {
      32 => {}                        // aA
      224 => {}                       // Aa
      _ => result.push(bytes[i - 1]), // TODO
    }
  }

  return result;
}
