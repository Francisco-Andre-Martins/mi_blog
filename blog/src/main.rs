use std::{fs,env};
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
    let args: Vec<String>=env::args().collect();
    if args.len()!=3{
        
        let num= args.len();
        println!("{num}");
        for arg in args{
            println!("{arg}")
        }
        println!("WRONG NUMBER OF ARGUMENTS FIRST IS THE INPUT DIRECTORY, SECOND THE OUTPUT");
        return
    }
    let paths = fs::read_dir(args[1].clone()).unwrap();

    let outputdir =&args[2];
    for path in paths {
            let mut new_path=path.unwrap();
            if !new_path.path().is_dir(){
                let mut name=  new_path.path().into_os_string().into_string().unwrap();
                let stuff=read_from_file(name.clone());
                let paragraphs: Vec<&str>= stuff.split('\n').collect();
                let validhtml: String= convert_into_post(paragraphs);
                let mut new_name=String::from(outputdir);
                new_name.push('/');
                new_name.push_str(&new_path.file_name().into_string().unwrap());
                new_name.push_str(".html");
                write_into_file(new_name, validhtml); 
            }



    }
    println!("this didn't blow up (surprisingly)");
}
