use rand::{self, Rng};

pub fn random_fg_color() -> String {
	use crossterm::style::{Color};
	
	let colors: Vec<String> = vec![
//        format!("{}", Colored::ForegroundColor(Color::White)),
			"white".to_string(),
    ];
    let c = &*rand::thread_rng().choose(&*colors).unwrap();
    c.clone()
}
