use std::char;
use std::env;

fn main() {
  let args : Vec<String> =  env::args().skip( 1 ).collect();
  let input = args.join( " " );
  let mut output = String::new();
  
  for chr in input.chars() {
    if chr >= '!' && chr <= '~' {
        output.push( char::from_u32( 0xff00 + (chr as u32) - 32 ).unwrap() );
        
        continue
    }
    
    output.push( chr );
  }
  
  println!( "{}", output );
}
