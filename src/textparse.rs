use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_text_file(filename: String) -> Vec<(String,String)> {


  let file = File::open(format!("src/quizzes/{}", filename)).expect("Error opening File");

  //file.read_to_string(&mut contents).expect("Unable to read to string");

  let reader = BufReader::new(file);

  let lines: Vec<_> = reader.lines().collect();

  let mut tuple_vector: Vec<(String,String)> = Vec::new();

  
  for l in lines {
    let mut split = l.as_ref().unwrap().split(":");

    let tuple = (String::from(split.next().unwrap()),             
    String::from(split.next().unwrap()));

    tuple_vector.push(tuple);
  }
  
  tuple_vector
  // let opcodes = contents.split(":");

  // println!("{:#?}",opcodes);
  
  
}