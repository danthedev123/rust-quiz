/* This code is under the MIT license, 
 * which is included in the source of this game.
*/

use std::io;
use std::fs;
use std::io::Write;

mod textparse;

struct Player {
  score: i32,
}

impl Player {
  fn get_score(&self) -> i32 {
    self.score
  }

  fn add_score(&mut self, a: i32) {
    let current_score = self.get_score();

    self.score = current_score + a;
  }
}

struct Game {
  player: Player,
  questions_correct: i32,
}

impl Game {

  

  fn ask_questions(&mut self, tuple_vector: Vec<(String,String)>) {
    
    for tuple in &tuple_vector {

      println!("{}?", tuple.0); // Print the question
      let mut answer = String::new(); // Get the answer

      io::stdin().read_line(&mut answer).expect("error: unable to read user input");      

      if answer.trim() == tuple.1.trim() {
        self.player.add_score(50);
        self.questions_correct += 1;
      } else if answer.trim() != tuple.1.trim() {
        println!("You are incorrect. The correct answer is {}", tuple.1);
        //continue;
      }
    }


    println!("Thanks for taking this quiz! You got {}/{} and your score is {}", self.questions_correct, tuple_vector.len(), self.player.get_score());

    if self.questions_correct == tuple_vector.len() as i32 {
      println!("
      \n _     _  _______  ___      ___        ______   _______  __    _  _______  __  \n| | _ | ||       ||   |    |   |      |      | |       ||  |  | ||       ||  | \n| || || ||    ___||   |    |   |      |  _    ||   _   ||   |_| ||    ___||  | \n|       ||   |___ |   |    |   |      | | |   ||  | |  ||       ||   |___ |  | \n|       ||    ___||   |___ |   |___   | |_|   ||  |_|  ||  _    ||    ___||__| \n|   _   ||   |___ |       ||       |  |       ||       || | |   ||   |___  __  \n|__| |__||_______||_______||_______|  |______| |_______||_|  |__||_______||__| \n                                             \n
      ")
    } else if self.questions_correct == 0 {
      println!("
\n  _______                             _                   __\n |__   __|                           (_)        _        / /\n    | |_ __ _   _    __ _  __ _  __ _ _ _ __   (_)______| | \n    | | \'__| | | |  / _` |/ _` |/ _` | | \'_ \\    |______| | \n    | | |  | |_| | | (_| | (_| | (_| | | | | |  _       | | \n    |_|_|   \\__, |  \\__,_|\\__, |\\__,_|_|_| |_| (_)      | | \n             __/ |         __/ |                         \\_\\\n            |___/         |___/                             \n
      ")
    } else {
      println!("
\n _____                 _   _              _   _   _                                     _   _                             \n|  __ \\               | | | |            | | | | | |                                   | | | |                            \n| |  \\/ ___   ___   __| | | |_ _ __ _   _| | | |_| | __ ___   _____    __ _ _ __   ___ | |_| |__   ___ _ __    __ _  ___  \n| | __ / _ \\ / _ \\ / _` | | __| \'__| | | | | |  _  |/ _` \\ \\ / / _ \\  / _` | \'_ \\ / _ \\| __| \'_ \\ / _ \\ \'__|  / _` |/ _ \\ \n| |_\\ \\ (_) | (_) | (_| | | |_| |  | |_| |_| | | | | (_| |\\ V /  __/ | (_| | | | | (_) | |_| | | |  __/ |    | (_| | (_) |\n \\____/\\___/ \\___/ \\__,_|  \\__|_|   \\__, (_) \\_| |_/\\__,_| \\_/ \\___|  \\__,_|_| |_|\\___/ \\__|_| |_|\\___|_|     \\__, |\\___/ \n                                     __/ |                                                                     __/ |      \n                                    |___/                                                                     |___/       \n
      ")
    }

    
    
  }
}

fn main() {

  println!("Welcome to....");

    println!("
\n ____  _     ____  _____    ____  _     _  ____  _ \n/  __\\/ \\ /\\/ ___\\/__ __\\  /  _ \\/ \\ /\\/ \\/_   \\/ \\\n|  \\/|| | |||    \\  / \\    | / \\|| | ||| | /   /| |\n|    /| \\_/|\\___ |  | |    | \\_\\|| \\_/|| |/   /_\\_/\n\\_/\\_\\\\____/\\____/  \\_/    \\____\\\\____/\\_/\\____/(_)\n                                                   \n                                          
    ");
    let mut selected_file = String::new();


    // TODO: Use selection (for example 1,2,3) instead of accepting arbitrary file name input.
    println!("Select one of the following files in the quizzes folder:");
    for file in fs::read_dir("./src/quizzes").unwrap() {
        println!("{}", file.unwrap().file_name().to_str().unwrap());
    }
    print!("Choose one: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut selected_file).expect("error: unable to read user input");

  
    let game_player: Player = Player {score: 0};

    let mut game: Game = Game {player: game_player, questions_correct: 0};

    let tuple_vector = textparse::read_text_file(selected_file.trim().to_string());

    game.ask_questions(tuple_vector);
}