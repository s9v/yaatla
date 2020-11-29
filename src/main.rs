use std::io::Write;
use std::cmp::min;

fn main() {
  // handles to stdin/stdout
  let mut sin = std::io::stdin();
  let mut sout = std::io::stdout();
  sout.flush();
  let hash = "0wefj2340".to_string();
  let hash_len = hash.len();
  println!("Memorize this: {}", hash);
  println!("       Length: {}", hash_len);
  println!();
  println!("Press [enter] to attempt to recall...");
  sin.read_line(&mut String::new());

  // clear screen
  for _ in 0..50 { println!(); }

  loop {
    // print censored hash
    print!("Memorize this: ");
    for _ in 0..hash_len { print!("*"); }
    println!();

    println!("       Length: {}", hash_len);

    // print recall prompt
    print!("  Recall here: ");
    sout.flush();

    // read attempted recall
    let mut recall = String::new();
    sin.read_line(&mut recall);
    recall.truncate(recall.trim_end().len());
    let rcll_len = recall.len();

    print!("       Result: ");
    for i in 0..min(hash_len, rcll_len) {
      if recall.as_bytes()[i] == hash.as_bytes()[i] { print!("."); }
      else { print!("!"); }
    }

    // print ?s if recall is longer than
    for _ in hash_len..rcll_len { print!("?"); }

    // print ?s if recall is shorter than
    for _ in rcll_len..hash_len { print!("?"); }
    println!();

    if recall == hash {
      println!("••• Recall complete!");
      break;
    }
    else {
      println!("••• Nope, try again...");
      println!();
    }
  }
}
