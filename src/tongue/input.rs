use libc;

use tongue::config::Config;
use tongue::{util, terminal, lexer, parser, evaluator};
    
const CTRL_B:    u8 = 2;
const CTRL_C:    u8 = 3;
const CTRL_D:    u8 = 4;
const CTRL_F:    u8 = 6;
const TAB:       u8 = 9;
const ENTER:     u8 = 13;
const CTRL_U:    u8 = 21;
const ESCAPE:    u8 = 27;
const BACKSPACE: u8 = 127;

pub fn read_from_stdin(config: &mut Config, orig_term: libc::termios) {
    let mut line = "".to_string();
    let ps1 = " $ ";
    util::stdout(ps1);
    loop {
        let mut buffer = [0; 4];
        util::stdin(&mut buffer);
        let c = buffer[0] as char;
        match buffer[0] {
            CTRL_B => {
                if line.len() == 0 {
                    continue;
                }
                let cursor_position = terminal::get_cursor_position();
                if cursor_position.x == ps1.len() {
                    continue;
                }
                util::stdout("\x1b[D");
            }
            CTRL_C => {
            }
            CTRL_D => {
                if line.len() == 0 {
                    util::stdout("\r\n");
                    break;
                }
            }
            CTRL_F => {
                if line.len() == 0 {
                    continue;
                }
                let cursor_position = terminal::get_cursor_position();
                if cursor_position.x == ps1.len() {
                    continue;
                }
                util::stdout("\x1b[C");
            }
            TAB => {
                
            }
            ENTER => {
                util::stdout("\r\n");
                terminal::tcsetattr(orig_term);
                let tokens = lexer::tokenize(line.as_str());
                let tree = parser::parse(tokens.clone());
                evaluator::eval(tree, config);
                terminal::enable_raw_mode();
                line = "".to_string();
                util::stdout(ps1);
            }
            CTRL_U => {
                util::stdout("\x1b[0K");
                line = "".to_string();
            }
            ESCAPE => {
                ;
            }
            BACKSPACE => {
                if line.len() == 0 {
                    continue;
                }
                let cursor_position = terminal::get_cursor_position();
                util::stdout(format!("\x1b[{};0H", cursor_position.y).as_str());
                util::stdout("\x1b[0K");
                util::stdout(ps1);
                line.remove(cursor_position.x-2-ps1.len());
                util::stdout(line.as_str());
                util::stdout(format!("\x1b[{};{}H", cursor_position.y, cursor_position.x-1).as_str());
            }
            _ => {
                let cursor_position = terminal::get_cursor_position();
                util::stdout(format!("\x1b[{};0H", cursor_position.y).as_str());
                util::stdout("\x1b[0K");
                util::stdout(ps1);
                line.insert(cursor_position.x-1-ps1.len(), c);
                util::stdout(line.as_str());
                util::stdout(format!("\x1b[{};{}H", cursor_position.y, cursor_position.x+1).as_str());
            }
        }
    }
}
