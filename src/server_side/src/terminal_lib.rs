use std::sync::{Mutex, LazyLock};
use std::io::{self, Write};

static DEBUG: LazyLock<Mutex<Vec<String>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

fn move_to(x: i8, y: i8) {
    print!("\x1b[{};{}H", y, x);
}

pub fn add_window(x: i8, y: i8, message: &str) {
    let lines: Vec<&str> = message.split('\n').collect();

    let mut width = 0;
    for line in &lines {
        if line.len() > width {
            width = line.len();
        }
    }

    width += 2;

    // top border
    move_to(x, y);
    print!("\x1b[48;5;238m");
    

    // message lines
    print!("\x1b[48;5;67m");
        
    print!("{}", " ".repeat(width+1));
    for (i, line) in lines.iter().enumerate() {

        move_to(x, y + 1 + i as i8);

        let padding = width - line.len();

        
        
        print!(" {}{}█", line, " ".repeat(padding));
        move_to(x, y + 2 + i as i8);
        
        
        
    }
    move_to(x, y + 1 + lines.len() as i8);
    print!(" {}█", " ".repeat(width));

    // bottom border
    move_to(x, y + 2 + lines.len() as i8);
    print!("\x1b[48;5;238m");
    print!(" {}", "▀".repeat(width+1));

    io::stdout().flush().unwrap();

    DEBUG
        .lock()
        .unwrap()
        .push(format!("window({}, {}) -> {}", x, y, message));
}

pub

fn text(x: i8, y: i8, message: &str) {
    move_to(x, y);
    print!("{}", message);
    io::stdout().flush().unwrap();

    DEBUG
        .lock()
        .unwrap()
        .push(format!("text({}, {}) -> {}", x, y, message));
}

fn debug_messages() {
    let mut y = 30;

    let log = DEBUG.lock().unwrap();

    for msg in log.iter() {
        move_to(2, y);
        print!("{}", msg);
        y += 1;
    }

    io::stdout().flush().unwrap();
}