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
         
        Ok(())

}
// ** todo
// let user input a name and description
// add to ingo to file system  (json in a txt ?
// * fiels: title, description, status
// let user check todos  by type
// let user change todo type
