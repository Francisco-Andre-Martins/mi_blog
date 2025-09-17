use std::{fs,env};
fn directory_creator()->String{
    String::from("<!DOCTYPE html> <html lang=\"pt\"> <head><meta charset=\"UTF-8\"><link rel=\"stylesheet\" href=\"../style.css\"><title>Stuff </title></head><body> <ul class=\"header\"><li><a href=\"./index.html\"> Francisco Martins</a></li><li><a href=\"../pages/directory.html\">Blog</a></li></ul><div class=\"content\"><h1>Posts</h1>")

}
fn add_link_to_directory(path:String, pagetitle:&str)->String{
    let mut return_string=String::new();
    return_string.push_str("<a href=");
    println!("{path}");
    return_string.push_str(&path);
    return_string.push_str(".html");

    return_string.push_str(">");
    return_string.push_str(&pagetitle);
    return_string.push_str("<a>");
    return return_string;
}
fn end_directory()->String{
    String::from("</div></body></html>")
}
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
    returnstuff.push_str("<!DOCTYPE html> <html lang=\"pt\"> <head><meta charset=\"UTF-8\"><link rel=\"stylesheet\" href=\"../style.css\"><title>Stuff </title></head><body><ul class=\"header\"><li><a href=\"./index.html\"> Francisco Martins</a></li><li><a href=\"../pages/directory.html\">Blog</a></li></ul><div class=\"content\">");
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
    returnstuff.push_str("</div></body></html>");
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
    let mut directory_page=directory_creator();

    //This part of the code will assemble the content pages
    for path in paths {
            let mut new_path=path.unwrap();
            if !new_path.path().is_dir(){
                let name=  new_path.path().into_os_string().into_string().unwrap();

                let stuff=read_from_file(name.clone());
                let paragraphs: Vec<&str>= stuff.split('\n').collect();

 
                let mut new_name=String::from(outputdir);
                new_name.push('/');
                new_name.push_str(&new_path.file_name().into_string().unwrap());
                new_name.push_str(".html");
                directory_page.push_str(&add_link_to_directory(String::from(&new_path.file_name().into_string().unwrap()),paragraphs[0]));
                let validhtml: String= convert_into_post(paragraphs);
                write_into_file(new_name, validhtml); 
            }
    }
    directory_page.push_str(&end_directory()); 
    let mut directory_path = String::from(outputdir);
    println!("I am yelling this {directory_path}");
    directory_path.push_str("/directory.html");
    write_into_file(directory_path, directory_page);
    //This part of the code assembles the directory
    //I first want to see what the default sort order is
    //sorted_paths.sort();

    println!("this didn't blow up (surprisingly)");
}
