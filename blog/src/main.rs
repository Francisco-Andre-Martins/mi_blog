use std::{fs};
//function: read from file (reads from given path, returns string)
fn read_from_file(path_to_read:String)->String{
    
    let contents = fs::read_to_string(path_to_read).expect("Should have been able to read the file");
    contents

}
//function: parse into paragraphs
// fn parse_into_paragraphs(stuff:String, backstuff:Vec<&str>)->Vec<&str>{
//     let returnstuff:Vec<&str> = stuff.split('\n').collect();
//     returnstuff
// }
//function: convert into HTML
fn convert_into_post(paragraphs: Vec<&str>)->String{
    let mut returnstuff= String::new();
    returnstuff.push_str("<!DOCTYPE html> <html lang=\"pt\"> <head><meta charset=\"UTF-8\"><link rel=\"stylesheet\" href=\"style.css\"><title>Stuff </title></head><body>");
    let mut isfirst=true;
    for el in paragraphs{
        if isfirst{
            returnstuff.push_str("<h1>");
            returnstuff.push_str(el);
            returnstuff.push_str("</h1>");
            isfirst=false;
        }else{
            returnstuff.push_str("<p>");
            returnstuff.push_str(el);
            returnstuff.push_str("</p>");
        }
    }
    returnstuff.push_str("</body></html>");
    returnstuff
}
//function: write into file
fn write_into_file(path:String,stuff:String){
    fs::write(path,stuff).expect("Should have been able to write to the file");
}
//    fs::read_dir();

//main: reads files from directory (supplied in .env or in arguments? assume . at first, NON RECURSIVE FOR STARTERS)
fn main() {
    let mut path = String::from("text.txt");
    let stuff=read_from_file(path.clone());
    let paragraphs: Vec<&str>= stuff.split('\n').collect();
    let validhtml: String= convert_into_post(paragraphs);
    path.push_str(".html");
    write_into_file(path, validhtml);
    println!("{stuff}");
    println!("Hello, world!");
}
