use std::io;
use std::path;

fn main() {
  let read_result: Result<@io::Reader, ~str>;
  read_result = io::file_reader(~path::Path("ghasper.torrent"));
  if read_result.is_ok() {
    let result = read_result.unwrap();

    let mut chars: ~[char];
    chars = result.read_chars(5);
    println(chars.to_str());
  }
}
