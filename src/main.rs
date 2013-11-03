use std::io;
use std::path;
use std::ascii::Ascii;

fn main() {
  let read_result: Result<@io::Reader, ~str>;
  /*let read_result: Result<~[u8], ~str>;*/
  /*read_result = io::read_whole_file(~path::Path("ghasper.torrent"));*/
  read_result = io::file_reader(~path::Path("ghasper.torrent"));
  /*let file = read_result.unwrap();*/
  /*let lines = file.read_byte();*/
  if read_result.is_ok() {
    let mut result = read_result.unwrap();

    let mut chars: ~[u8] = ~[0, 0, 0, 0, 0];
    result.read(chars, 5);
    /*let datchar = result.read_byte().to_u8().to_ascii();*/
    println(chars.to_str());
    /*let datchar = result[0].to_ascii();*/
    /*println(datchar.to_str());*/
    /*let dischar = result.shift().to_ascii();*/
    /*if dischar.to_str() == ~"d" {*/
      /*println("yeahhh dict");*/
    /*}*/
    /*println(dischar.to_str());*/
  }

}
