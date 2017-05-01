use std::collections::HashMap;
use ::color::Color;
use ::statusline::StatusLine;
use ::blocks::Widget;

#[derive(Clone, Debug, Serialize)]
pub struct BarConfiguration {
    pub refresh_interval: usize,
    pub widgets: HashMap<String, WidgetConfig>
}

#[derive(Clone, Debug, Serialize)]
pub struct WidgetConfig {
    pub widget_type: WidgetType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<WidgetStyle>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WidgetType {
    Clock(String),
    Text(String),
    Shell(String),
    FlorpBlarg
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WidgetStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    foreground: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    separator_width: Option<usize>
}

impl Default for WidgetStyle {
    fn default() -> Self {
        WidgetStyle {
            foreground: None,
            background: None,
            border: None,
            separator: None,
            separator_width: None
        }
    }
}

impl BarConfiguration {
    pub fn new(refresh_interval: usize) -> Self {
        BarConfiguration {
            refresh_interval: refresh_interval,
            widgets: HashMap::new()
        }
    }

    pub fn generate(self) -> StatusLine {
        let mut line = StatusLine::new();

        for (_, widget) in self.widgets.into_iter() {

        }

        line
    }
}

impl WidgetConfig {
    pub fn new(ty: WidgetType) -> Self {
        WidgetConfig {
            widget_type: ty,
            style: Some(WidgetStyle {
                foreground: Some(::color::BLACK),
                ..Default::default()
            })
        }
    }

    pub fn generate(self) -> Result<Box<Widget + Send + 'static>, &'static str> {
        let widget: Box<Widget + Send + 'static> = match self.widget_type {
            WidgetType::FlorpBlarg => Box::new(::blocks::FlorpBlarg::new()),
            WidgetType::Shell(cmd) => Box::new(::blocks::Shell::new(cmd)),
            WidgetType::Clock(fmt) => Box::new(::blocks::Clock::new(&*fmt).map_err(|_|"Invalid format")?),
            WidgetType::Text(text) => Box::new(text),
        };

        Ok(widget)
    }
}