use super::{Widget, Block};
use ::i3::Button;
use std::borrow::Cow;

pub struct FlorpBlarg {
    data: Block,
    flag: bool,
}

impl FlorpBlarg {
    pub fn new() -> Self {
        FlorpBlarg {
            flag: false,
            data: Block {
                name: Some("FlorpBlarg".into()),
                instance: Some("FlorpBlarg".into()),
                full_text: "FLORP".into(),
                foreground_color: Some(::color::CRIMSON),
                background_color: Some(::color::BLACK),
                ..Default::default()
            },
        }
    }
}

impl Widget for FlorpBlarg {
    fn update<'a>(&'a mut self) -> Cow<'a, Block> {
        Cow::Borrowed(&self.data)
    }

    fn get_name(&self) -> Option<&str> {
        Some("FlorpBlarg")
    }

    fn get_instance(&self) -> Option<&str> {
        Some("FlorpBlarg")
    }

    fn handle_event(&mut self, event: Button) {
        if self.flag {
            self.data.foreground_color = Some(::color::CRIMSON);
            self.data.background_color = Some(::color::BLACK);
            self.data.full_text = "FLORP".into();
        } else {
            self.data.foreground_color = Some(::color::BLACK);
            self.data.background_color = Some(::color::CRIMSON);
            self.data.full_text = "BLARG".into();

        }
        self.flag = !self.flag;
    }
}