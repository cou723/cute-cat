use std::env;

const CAT_SUCCESS: &str = r"(=^･ω ･^=)";
const CAT_FAILED: &str = r"(=^･ｪ･^=)";

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_text;
    if args.len() == 1 {
        print!("{CAT_FAILED}");
        return;
    }

    match std::fs::read_to_string(args[1].clone()) {
        Ok(text) => file_text = text,
        Err(_e) => {
            print!("{CAT_SUCCESS}? file not found");
            return;
        }
    }
    let file_width = file_text.lines().map(|line| line.len()).max().unwrap();

    print_cat(&file_text, file_width);
}

fn print_cat(file_text: &str, file_width: usize) {
    print_horizontal_bar(file_width);
    for line in file_text.lines() {
        print!("|   {}", line);
        for _ in 0..(file_width - line.len()) {
            print!(" ");
        }
        println!("    |");
    }
    print_horizontal_bar(file_width);
}

fn print_horizontal_bar(file_width: usize) {
    for i in 0..file_width {
        if i == file_width / 2 {
            print!("{}", CAT_SUCCESS);
        } else {
            print!("-");
        }
    }
    println!("");
}
