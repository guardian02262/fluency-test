use sha2::{Digest, Sha256};

/// data structure to hold return values of our pow calculation
#[derive(Debug, PartialEq)]
struct SimplePow {
    hash: String,
    prefix: String
}

/// at the moment program will panic if input string is not valid 
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_base64 = &args[1];

    let r = simple_pow(input_base64).unwrap_or(SimplePow { hash: "none".to_string(), prefix: "none".to_string()});

    println!("{}\n{}", r.hash, r.prefix);
}

/// brute force looping though all values from [0,0,0,0] to [255, 255, 255, 255]
fn simple_pow(input_base64: &String) -> Option<SimplePow> {
    let input_as_bytes = &hex::decode(input_base64).unwrap()[..];

    for i1 in 0..256 {
        for i2 in 0..256 {
            for i3 in 0..256 {
                for i4 in 0..256 {
                    let r = hash_with_prefix(input_as_bytes, &vec![i1 as u8, i2 as u8, i3 as u8, i4 as u8][..]);
                    match r {
                        Some(_) => return r,
                        None => continue
                    }
                }
            }
        }
    }

    None
}

/// this is function which makes all the calculations and checking the last two bytes 
/// if we get the expected result
fn hash_with_prefix(input: &[u8], prefix: &[u8]) -> Option<SimplePow> {
    let mut hasher = Sha256::new_with_prefix(prefix);
    hasher.update(input);
    let res = &hasher.finalize()[..];    
    let second_last = &res[&res.len() - 2];
    let last = &res[&res.len() - 1];
    // 0xca = 202, 0xfe = 254
    if *second_last == 202 && *last == 254 {
        return Some(SimplePow {
            hash: hex::encode(res),
            prefix: hex::encode(prefix)
        });
    } 

    None
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn right_input() {
      let r= simple_pow(&"129df964b701d0b8e72fe7224cc71643cf8e000d122e72f742747708f5e3bb6294c619604e52dcd8f5446da7e9ff7459d1d3cefbcc231dd4c02730a22af9880c".to_string());
      let expected_result = SimplePow {
          hash: "6681edd1d36af256c615bf6dcfcda03c282c3e0871bd75564458d77c529dcafe".to_string(),
          prefix: "00003997".to_string()
      };
      assert_eq!(r, Some(expected_result));
  }

  #[test]
  fn wrong_input() {
      let r= simple_pow(&"329df964b701d0b8e72fe7224cc71643cf8e000d122e72f742747708f5e3bb6294c619604e52dcd8f5446da7e9ff7459d1d3cefbcc231dd4c02730a22af9880c".to_string());
      let expected_result = SimplePow {
          hash: "6681edd1d36af256c615bf6dcfcda03c282c3e0871bd75564458d77c529dcafe".to_string(),
          prefix: "00003997".to_string()
      };
      assert_ne!(r, Some(expected_result));
  }

  #[test]
  #[should_panic]
  fn incorrect_input_should_panic() {
      let r= simple_pow(&"s".to_string());
      let expected_result = SimplePow {
          hash: "6681edd1d36af256c615bf6dcfcda03c282c3e0871bd75564458d77c529dcafe".to_string(),
          prefix: "00003997".to_string()
      };
      assert_ne!(r, Some(expected_result));
  }
}