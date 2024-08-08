use std::env;
    
fn text_wrap(text: &String, width: usize) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    let mut s_buf: String = "".to_string();
    for word in text.split(" ") {
        if s_buf.len() + word.len() >= width {
            s_buf.pop();
            lines.push(s_buf);
            s_buf = "".to_string();
        }
        s_buf.push_str(&word);
        s_buf.push_str(" ");
    }
    lines.push(s_buf);
    lines
}

fn ralsay(text: String) {
    let art: &'static str = "
               -^- 
           _/\\/_  \\/\\_
          (____ ))____)
           / -     - \\
          / ( ^)-(^ ) \\
         / ____v-v____ \\
         \\(-    _-  __)/
           \\-  -__---/
           /  / V \\   \\
          /__/\\   /    \\
         =      V       =
       _=_              _=_
         -=---______---=-
            _| | | |_
           (___- -___)
    ";

    let initial_width: usize = 40;

    let wrapped: Vec<String> = text_wrap(&text, initial_width);
    let mut max_length: usize = 0;

    for line in &wrapped {
	if line.len() > max_length {
	    max_length = line.len()
	}
    }

    let box_width: usize = max_length + 2;
    println!(" {}", "_".repeat(box_width),);
    println!("/{}\\", " ".repeat(box_width));

    for line in &wrapped {
	println!("| {}{} |", line, " ".repeat(box_width - line.len() - 2));
    }

    println!("\\{}/", " ".repeat(box_width));
    println!(" {}", "=".repeat(box_width));
    println!("{}", art);
}

fn check_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
	eprintln!("You seem to have not written your message. Ralsei will therefore not respond to your command.");
	std::process::exit(1);
    }
    args.remove(0);

    args
}

fn main() {
    let args: Vec<String> = check_args();
    ralsay(args.join(" "));
}
