use rand::{self, Rng};

use crossterm::style::{Color, SetForegroundColor};

pub fn random_fg_color() -> String {
	
	let colors: Vec<String> = vec![
        SetForegroundColor(Color::White).to_string(),
        SetForegroundColor(Color::Red).to_string(),
        //SetForegroundColor(Color::Blue).to_string(),
        SetForegroundColor(Color::Yellow).to_string(),
        SetForegroundColor(Color::Green).to_string(),
        SetForegroundColor(Color::Cyan).to_string(),
    ];
    let c = &*rand::thread_rng().choose(&*colors).unwrap();
    c.clone()
}
