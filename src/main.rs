fn main() {
    let input = get_input();    
    println!("{input}");
}

fn get_input() -> String {
    use std::io;
    
    print!("> ");
    io::Write::flush(&mut io::stdout()).expect("Error when flushing");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error with input reading.");
    
    return input;
}