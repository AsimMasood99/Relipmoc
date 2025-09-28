use std::fs;
use std::path::Path;

mod lexer;
mod parser;

fn get_code() -> String {
    // TODO: allow custom file path
    // let mut file_name = String::new();
    // print!("Enter the file name: ");
    // io::stdout().flush().unwrap(); // flush to print on screen
    // io::stdin().read_line(&mut file_name).unwrap();

    let path = Path::new("data").join("code.txt");
    let file_content = fs::read_to_string(path).unwrap();

    return file_content;
}


fn main() {
    let code = get_code();
    let tokens = lexer::lexer::lex(code);
    
    println!("{:?}\n\n", tokens);

    let ast = parser::parser::parser(tokens);

    println!("{:#?}\n\n", ast);
}
