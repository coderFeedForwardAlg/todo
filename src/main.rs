use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
       let mut conn = File::open("workData.txt")?;
        let mut content = String::new();
        conn.read_to_string(&mut content)?;
        
        content = content.trim().to_string();
        let num = match content.parse::<i32>(){
            Ok(num) => {num}
            Err(e) => {
                println!("error {} ", e);
                0
            }
        };
        println!(" the number is {} ", num);
        println!("the file says {} ", content);


        println!("how many pushups did you do");
        let mut pushups = String::new();
        io::stdin().read_line(&mut pushups).expect("cant read line");
        pushups = pushups.trim().to_string();
        let new_pushups = match pushups.parse::<i32>(){
            Ok(num) => {num}
            Err(e) => {
                println!("error with user input {} ", e);
                0
            }
        };
        let mut workData = File::create("workData.txt")?;
        workData.write_all((num + new_pushups).to_string().as_bytes())?;
        
        println!("what did you learn");
        let mut learned_user_input = String::new();
        io::stdin().read_line(&mut learned_user_input).expect("cant read line");
        let mut learned_file = File::options().append(true).open("learned.txt")?;

        learned_file.write_all(learned_user_input.to_string().as_bytes())?;
        //writeln!(&mut learned_file, learned_user_input)?;
        
        println!("what do you still need to learn / look back at");
        let mut need_to_learn_input = String::new();
        io::stdin().read_line(&mut need_to_learn_input).expect("cant read line");
        let mut need_learn_file = File::options().append(true).open("need_learn.txt")?;
        need_learn_file.write_all(need_to_learn_input.to_string().as_bytes())?;
        

        Ok(())

}
// ** todo
// let user input a name and description
// add to ingo to file system  (json in a txt ?
// * fiels: title, description, status
// let user check todos  by type
// let user change todo type
