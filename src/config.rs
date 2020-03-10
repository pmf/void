use std::{
    collections::HashMap,
    env, fmt,
    fs::File,
    io::{self, Error, ErrorKind, Read},
};

use crossterm::{
    event
};

#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Action {
    LeftClick(u16, u16),
    RightClick(u16, u16),
    Release(u16, u16),
    Char(char),
    UnselectRet,
    ScrollUp,
    ScrollDown,
    DeleteSelected,
    SelectUp,
    SelectDown,
    SelectLeft,
    SelectRight,
    EraseChar,
    CreateSibling,
    CreateChild,
    CreateFreeNode,
    ExecSelected,
    DrillDown,
    PopUp,
    PrefixJump,
    ToggleCompleted,
    ToggleHideCompleted,
    Arrow,
    AutoArrange,
    ToggleCollapsed,
    Quit,
    Save,
    ToggleShowLogs,
    EnterCmd,
    FindTask,
    YankPasteNode,
    RaiseSelected,
    LowerSelected,
    Search,
    UndoDelete,
    Help,
    SelectParent,
    SelectNextSibling,
    SelectPrevSibling,
}

fn to_action(input: String) -> Option<Action> {
    match &*input {
        "unselect" => Some(Action::UnselectRet),
        "scroll_up" => Some(Action::ScrollUp),
        "scroll_down" => Some(Action::ScrollDown),
        "delete" => Some(Action::DeleteSelected),
        "select_up" => Some(Action::SelectUp),
        "select_down" => Some(Action::SelectDown),
        "select_left" => Some(Action::SelectLeft),
        "select_right" => Some(Action::SelectRight),
        "erase" => Some(Action::EraseChar),
        "create_sibling" => Some(Action::CreateSibling),
        "create_child" => Some(Action::CreateChild),
        "create_free_node" => Some(Action::CreateFreeNode),
        "execute" => Some(Action::ExecSelected),
        "drill_down" => Some(Action::DrillDown),
        "pop_up" => Some(Action::PopUp),
        "jump" => Some(Action::PrefixJump),
        "toggle_completed" => Some(Action::ToggleCompleted),
        "toggle_hide_completed" => Some(Action::ToggleHideCompleted),
        "arrow" => Some(Action::Arrow),
        "auto_arrange" => Some(Action::AutoArrange),
        "toggle_collapsed" => Some(Action::ToggleCollapsed),
        "quit" => Some(Action::Quit),
        "save" => Some(Action::Save),
        "toggle_show_logs" => Some(Action::ToggleShowLogs),
        "enter_command" => Some(Action::EnterCmd),
        "find_task" => Some(Action::FindTask),
        "yank_paste_node" => Some(Action::YankPasteNode),
        "raise_selected" => Some(Action::RaiseSelected),
        "lower_selected" => Some(Action::LowerSelected),
        "search" => Some(Action::Search),
        "undo_delete" => Some(Action::UndoDelete),
        "help" => Some(Action::Help),
        "select_parent" => Some(Action::SelectParent),
        "select_next_sibling" => Some(Action::SelectNextSibling),
        "select_prev_sibling" => Some(Action::SelectPrevSibling),
        _ => None,
    }
}

// Alt and Control must be specified with capital letters C- and A-
fn to_key(raw_key: String) -> Option<event::KeyCode> {
    fn extract_key(raw_key: &str, idx: usize) -> Option<char> { raw_key.chars().nth(idx) }

    match &*raw_key {
        "esc" => Some(event::KeyCode::Esc),
        "pgup" => Some(event::KeyCode::PageUp),
        "pgdn" => Some(event::KeyCode::PageDown),
        "del" => Some(event::KeyCode::Delete),
        "backspace" => Some(event::KeyCode::Backspace),
        "up" => Some(event::KeyCode::Up),
        "down" => Some(event::KeyCode::Down),
        "left" => Some(event::KeyCode::Left),
        "right" => Some(event::KeyCode::Right),

        "space" => Some(event::KeyCode::Char(' ')),
        "enter" => Some(event::KeyCode::Char('\n')),
        "tab" => Some(event::KeyCode::Char('\t')),

        key if key.len() == 1 => extract_key(key, 0).map(event::KeyCode::Char),

        //key if key.starts_with("A-") => extract_key(key, 2).map(event::KeyCode::Alt),
        //key if key.starts_with("C-") => extract_key(key, 2).map(event::KeyCode::Ctrl),

        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct KeyWithModifiers {
	code: event::KeyCode,
	modifiers: event::KeyModifiers,
}

#[derive(Debug, Clone)]
pub struct Config {
    config: HashMap<KeyWithModifiers, Action>,
}

impl Default for Config {
    fn default() -> Config {
        use crossterm::event::KeyCode::*;
        Config {
            config: vec![
                (KeyWithModifiers { code: Esc,        modifiers: event::KeyModifiers::empty()}, Action::UnselectRet),
                (KeyWithModifiers { code: PageUp,     modifiers: event::KeyModifiers::empty()}, Action::ScrollUp),
                (KeyWithModifiers { code: PageDown,   modifiers: event::KeyModifiers::empty()}, Action::ScrollDown),
                (KeyWithModifiers { code: Delete,     modifiers: event::KeyModifiers::empty()}, Action::DeleteSelected),
                (KeyWithModifiers { code: Up,         modifiers: event::KeyModifiers::empty()}, Action::SelectUp),
                (KeyWithModifiers { code: Down,       modifiers: event::KeyModifiers::empty()}, Action::SelectDown),
                (KeyWithModifiers { code: Left,       modifiers: event::KeyModifiers::empty()}, Action::SelectLeft),
                (KeyWithModifiers { code: Right,      modifiers: event::KeyModifiers::empty()}, Action::SelectRight),
                (KeyWithModifiers { code: Backspace,  modifiers: event::KeyModifiers::empty()}, Action::EraseChar),
                (KeyWithModifiers { code: F(1),       modifiers: event::KeyModifiers::empty()}, Action::PrefixJump),
                (KeyWithModifiers { code: Enter,      modifiers: event::KeyModifiers::empty()}, Action::CreateSibling),
                (KeyWithModifiers { code: Tab,        modifiers: event::KeyModifiers::empty()}, Action::CreateChild),
                (KeyWithModifiers { code: Char('n'),  modifiers: event::KeyModifiers::CONTROL}, Action::CreateFreeNode),
                (KeyWithModifiers { code: Char('k'),  modifiers: event::KeyModifiers::CONTROL}, Action::ExecSelected),
                (KeyWithModifiers { code: Char('w'),  modifiers: event::KeyModifiers::CONTROL}, Action::DrillDown),
                (KeyWithModifiers { code: Char('q'),  modifiers: event::KeyModifiers::CONTROL}, Action::PopUp),
                (KeyWithModifiers { code: Char('f'),  modifiers: event::KeyModifiers::CONTROL}, Action::PrefixJump),
                (KeyWithModifiers { code: Char('a'),  modifiers: event::KeyModifiers::CONTROL}, Action::ToggleCompleted),
                (KeyWithModifiers { code: Char('h'),  modifiers: event::KeyModifiers::CONTROL}, Action::ToggleHideCompleted),
                (KeyWithModifiers { code: Char('r'),  modifiers: event::KeyModifiers::CONTROL}, Action::Arrow),
                (KeyWithModifiers { code: Char('p'),  modifiers: event::KeyModifiers::CONTROL}, Action::AutoArrange),
                (KeyWithModifiers { code: Char('t'),  modifiers: event::KeyModifiers::CONTROL}, Action::ToggleCollapsed),
                (KeyWithModifiers { code: Char('c'),  modifiers: event::KeyModifiers::CONTROL}, Action::Quit),
                (KeyWithModifiers { code: Char('x'),  modifiers: event::KeyModifiers::CONTROL}, Action::Save),
                (KeyWithModifiers { code: Char('l'),  modifiers: event::KeyModifiers::CONTROL}, Action::ToggleShowLogs),
                (KeyWithModifiers { code: Char('e'),  modifiers: event::KeyModifiers::CONTROL}, Action::EnterCmd),
                (KeyWithModifiers { code: Char('v'),  modifiers: event::KeyModifiers::CONTROL}, Action::FindTask),
                (KeyWithModifiers { code: Char('y'),  modifiers: event::KeyModifiers::CONTROL}, Action::YankPasteNode),
                (KeyWithModifiers { code: Char('g'),  modifiers: event::KeyModifiers::CONTROL}, Action::RaiseSelected),
                (KeyWithModifiers { code: Char('d'),  modifiers: event::KeyModifiers::CONTROL}, Action::LowerSelected),
                (KeyWithModifiers { code: Char('u'),  modifiers: event::KeyModifiers::CONTROL}, Action::Search),
                (KeyWithModifiers { code: Char('z'),  modifiers: event::KeyModifiers::CONTROL}, Action::UndoDelete),
                (KeyWithModifiers { code: Char('?'),  modifiers: event::KeyModifiers::CONTROL}, Action::Help),
                (KeyWithModifiers { code: Char('P'),  modifiers: event::KeyModifiers::ALT}, Action::SelectParent),
                (KeyWithModifiers { code: Char('n'),  modifiers: event::KeyModifiers::ALT}, Action::SelectNextSibling),
                (KeyWithModifiers { code: Char('p'),  modifiers: event::KeyModifiers::ALT}, Action::SelectPrevSibling),
            ]
            .into_iter()
            .collect(),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Configured Hotkeys:").unwrap();
        for (key, action) in &self.config {
            writeln!(f, "    {:?}: {:?}", action, key).unwrap();
        }
        Ok(())
    }
}

impl Config {
    pub fn maybe_parsed_from_env() -> io::Result<Config> {
        if let Ok(p) = env::var("KEYFILE") {
            Config::parse_keyfile(p)
        } else {
            Ok(Config::default())
        }
    }

    pub fn parse_keyfile(p: String) -> io::Result<Config> {
        let mut buf = String::new();
        let mut f = File::open(p)?;
        f.read_to_string(&mut buf)?;
        let mut config = Config::default();
        for (mut line_num, line) in buf.lines().enumerate() {
            if line == "" || line.starts_with('#') {
                continue;
            }

            // Zero based indexing inappropriate here.
            line_num += 1;

            let parts: Vec<_> = line.splitn(2, ':').map(|p| p.trim()).collect();
            if parts.len() != 2 {
                let e = format!("No colon found on line {}", line_num);
                error!("{}", e);
                return Err(Error::new(ErrorKind::Other, e));
            }

            let (raw_action, raw_key) = (parts[0], parts[1]);

            let key_opt = to_key(raw_key.to_owned());
            let action_opt = to_action(raw_action.to_owned());

            if key_opt.is_none() || action_opt.is_none() {
                let e = format!("invalid config at line {}: {}", line_num, line);
                error!("{}", e);
                return Err(Error::new(ErrorKind::Other, e));
            }

            let key = key_opt.unwrap();
            let action = action_opt.unwrap();

			// TODO: no modifiers for now ...
            config.config.insert(KeyWithModifiers { code: key, modifiers: event::KeyModifiers::empty() }, action);
        }

        Ok(config)
    }

    pub fn map(&self, e: event::Event) -> Option<Action> {
        match e {
            event::Event::Key(event::KeyEvent{code: code, modifiers: modifiers}) => {
				let k = KeyWithModifiers {code: code, modifiers: modifiers};
                if let Some(action) = self.config.get(&k).cloned() {
                    Some(action)
                } else {
					match code {
						event::KeyCode::Char(c) => Some(Action::Char(c)),
						other => None,
					}
                }
            },
			event::Event::Mouse(event::MouseEvent::Down(event::MouseButton::Right, x, y, modifiers)) =>
				Some(Action::RightClick(x, y)),
			event::Event::Mouse(event::MouseEvent::Down(event::MouseButton::Left, x, y, modifiers)) =>
				Some(Action::LeftClick(x, y)),
			event::Event::Mouse(event::MouseEvent::Up(_, x, y, modifiers)) =>
				Some(Action::Release(x, y)),
            event::Event::Key(other) => {
                warn!("Weird event {:?}", other);
				None
            },
            other => {
                warn!("Unknown event received: {:?}", other);
                None
            },
        }
    }
}
