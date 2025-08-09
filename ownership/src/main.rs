fn main() {
   let s = String::from("hello world");

   let word = first_word(&s);   

   println!("{word}");
}

fn first_word(s: &String) -> &str {
   let bytes = s.as_bytes();

   for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {         
         return &s[0..i];
      }
   }

   &s[..]
}

// fn dangle() -> String {
//    let s = String::from("hello");
//    s
// }

// fn calculate_length(s: &String) -> usize {
//    s.len()
// }

// fn change(some_string: &mut String) {
//    some_string.push_str(", World")
// }
