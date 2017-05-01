extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate time;

use clap::{Arg, App, SubCommand};
use blocks::*;
use i3::*;
use std::time::Duration;
use std::borrow::Cow;

pub mod color;
mod config;
mod i3;
mod blocks;
mod statusline;
mod infinite_array;

struct Counter(usize);
impl Widget for Counter {
    fn update(&mut self) -> Cow<'static, Block> {
        // self.0 += 1;
        Cow::Owned(Block {
            full_text: self.0.to_string(),
            name: Some("counter".into()),
            instance: Some("counter".into()),
            ..Default::default()
        })
    }

    fn get_name(&self) -> Option<&str> {
        Some("counter")
    }

    fn get_instance(&self) -> Option<&str> {
        Some("counter")
    }

    fn handle_event(&mut self, event: Button) {
        match event {
            Button::ScrollUp => self.0 = self.0.wrapping_add(1),
            Button::ScrollDown => self.0 = self.0.wrapping_sub(1),
            _ => {}
        }
    }

}

fn list_colors() {
    for (name, color) in ::color::COLORS.iter() {
        println!("{}\t{}", color, name.0);
    }
}

use config::*;

fn main() {
    let matches = App::new("rusty3bar")
                          .version("0.1")
                          .author("Gilad Naaman <gilad.naaman@gmail.com>")
                          .about("A status bar emitter for i3wm.")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                          .subcommand(SubCommand::with_name("print-colors")
                                      .about("Prints the supported list of named colors."))
                          .get_matches();
    
    if let Some(matches) = matches.subcommand_matches("print-colors") {
        list_colors();
        return;
    }
    let mut line = statusline::StatusLine::new();
    
    // let mut x = BarConfiguration::new(1);
    // x.widgets.insert("Gordon".into(), Widget::new(WidgetType::Clock("%d-%m-%Y".into())));
    // x.widgets.insert("Maurice".into(), Widget::new(WidgetType::Clock("%d-%m-%Y".into())));
    // println!("{}", serde_json::to_string(&x).unwrap());
    // println!("{}", serde_yaml::to_string(&x).unwrap());

    line.add("Simple block");

    line.add(Block {
                 full_text: "<b>E</b> = M&#169;<sup>2</sup>".into(),
                 foreground_color: Some(color::CRIMSON),
                 markup_type: MarkupType::Pango,
                 ..Default::default()
             });

    line.add(FlorpBlarg::new());

    line.add(Counter(0));

    line.add(Shell::new("echo $(whoami) @ $(hostname)"));

    line.add(Clock::new("%d-%m-%Y").unwrap());

    line.add(Clock::new("%H:%M").unwrap());

    line.run(Duration::from_secs(1));
}
