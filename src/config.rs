use std::{
    collections::HashMap,
    env, fmt,
    fs::File,
    io::{self, Error, ErrorKind, Read},
};

use crossterm::{
    event
};

/*
use termion::event::{Event, Key, MouseEvent};
*/

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

#[derive(Debug, Clone)]
pub struct Config {
    config: HashMap<event::KeyCode, Action>,
}

impl Default for Config {
    fn default() -> Config {
        use crossterm::event::KeyCode::*;
        Config {
            config: vec![
                (Esc, Action::UnselectRet),
                (PageUp, Action::ScrollUp),
                (PageDown, Action::ScrollDown),
                (Delete, Action::DeleteSelected),
                (Up, Action::SelectUp),
                (Down, Action::SelectDown),
                (Left, Action::SelectLeft),
                (Right, Action::SelectRight),
                (Backspace, Action::EraseChar),
                (F(1), Action::PrefixJump),
                (Char('\n'), Action::CreateSibling),
                (Char('\t'), Action::CreateChild),
				/*
                (Ctrl('n'), Action::CreateFreeNode),
                (Ctrl('k'), Action::ExecSelected),
                (Ctrl('w'), Action::DrillDown),
                (Ctrl('q'), Action::PopUp),
                (Ctrl('f'), Action::PrefixJump),
                (Ctrl('a'), Action::ToggleCompleted),
                (Ctrl('h'), Action::ToggleHideCompleted),
                (Ctrl('r'), Action::Arrow),
                (Ctrl('p'), Action::AutoArrange),
                (Ctrl('t'), Action::ToggleCollapsed),
                (Ctrl('c'), Action::Quit),
                (Ctrl('x'), Action::Save),
                (Ctrl('l'), Action::ToggleShowLogs),
                (Ctrl('e'), Action::EnterCmd),
                (Ctrl('v'), Action::FindTask),
                (Ctrl('y'), Action::YankPasteNode),
                (Ctrl('g'), Action::RaiseSelected),
                (Ctrl('d'), Action::LowerSelected),
                (Ctrl('u'), Action::Search),
                (Ctrl('z'), Action::UndoDelete),
                (Ctrl('?'), Action::Help),
                (Alt('P'), Action::SelectParent),
                (Alt('n'), Action::SelectNextSibling),
                (Alt('p'), Action::SelectPrevSibling),
				*/
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

            config.config.insert(key, action);
        }

        Ok(config)
    }

    pub fn map(&self, e: event::Event) -> Option<Action> {
        //use termion::event::{Key::*, MouseButton};
        match e {
            event::Event::Key(event::KeyEvent{code: code, modifiers: modifiers}) => {
                if let Some(action) = self.config.get(&code).cloned() {
                    Some(action)
                } else {
                    Some(Action::Char('a'))
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
