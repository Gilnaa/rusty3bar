use super::*;
use color::named;
use std::process::Command;
use std::borrow::Cow;

pub struct Shell {
    shell: String,
    command: String,
}

impl Shell {
    pub fn new<S: Into<String>>(cmd: S) -> Shell {
        Shell::new_with_shell("sh", cmd)
    }

    pub fn new_with_shell<SS: Into<String>, SC: Into<String>>(shell: SS, cmd: SC) -> Shell {
        Shell {
            shell: shell.into(),
            command: cmd.into(),
        }
    }
}

impl BlockProducer for Shell {
    fn update(&mut self) -> Cow<'static, Block> {
        let err_block = Block {
            full_text: "SHELL ERROR".into(),
            foreground_color: Some(named::RED),
            ..Block::default()
        };

        let res = if let Ok(output) = Command::new(self.shell.clone())
               .arg("-c")
               .arg(self.command.clone())
               .output() {
            if output.status.success() {
                Block {
                    full_text: String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .to_string(),
                    ..Block::default()
                }
            } else {
                err_block
            }
        } else {
            err_block
        };

        Cow::Owned(res)
    }
}
