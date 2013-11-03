use std::str::utf16_chars;
use std::ascii::Ascii;

pub fn decode(encoded: ~str) {
  println("decoding");
  println(encoded);
}

pub fn parse_string(string: &[u16]) -> ~str {
  utf16_chars(string, |ch| { println(ch.to_str()) });
  return ~"";
}

#[test]
fn test_parse_string() {
  /*assert!(parse_string(~[50, 60]) == ~"abc");*/
  let s = "heyyy";
  match from_str::<u16>(s) {
    Some(vec) => {
      println(fmt!("%?", vec));
    },
    None => println("wtf")
  }
}
