//! Structures describing the primitives used to communicate with i3bar.

use super::color::*;

/// A single display block data sent to i3bar.
#[derive(Debug, Clone, Serialize)]
pub struct Block {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    pub full_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "color")]
    pub foreground_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "background")]
    pub background_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "border")]
    pub border_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_width: Option<Padding>,

    pub align: Alignement,

    #[serde(rename = "urgent")]
    pub is_urgent: bool,

    pub separator: bool,

    #[serde(rename = "separator_block_width")]
    pub separator_width: usize,

    #[serde(rename = "markup")]
    pub markup_type: MarkupType,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            name: None,
            instance: None,
            full_text: "".into(),
            short_text: None,

            foreground_color: None,
            background_color: None,
            border_color: None,

            min_width: None,
            align: Alignement::Left,
            is_urgent: false,
            separator: true,
            separator_width: 9,
            markup_type: MarkupType::None,
        }
    }
}

/// Used to specify the minimum width of a block.
#[derive(Debug, Clone, Serialize)]
pub enum Padding {
    /// The minimum size of the block in pixels
    PixelSize(usize),

    /// Forces the block to be at least as wide as the given string.
    StringSize(String),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignement {
    Left,
    Right,
    Center,
}

/// Specifies the markup type used by i3bar.
/// The markup types defines how i3bar parses and presents the block's text.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkupType {
    /// Do not use any markup, present the text as is.
    None,

    /// Use Pango markup.
    /// See https://developer.gnome.org/pango/stable/PangoMarkupFormat.html
    Pango,
}

/// A click event sent from i3bar.
#[derive(Deserialize, Clone, Debug)]
pub struct I3BarEvent {
    /// The name of the clicked block.
    pub name: Option<String>,

    /// The instance name of the clicked block.
    pub instance: Option<String>,

    /// The button used to clock.
    pub button: Button,

    /// The X position of the mouse cursor relative to something.
    pub x: usize,

    /// The Y position of the mouse cursor relative to something.
    pub y: usize,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(from="usize")]
pub enum Button {
    Left,
    Middle,
    Right,
    ScrollUp,
    ScrollDown,
    Other(usize),
}

impl From<usize> for Button {
    fn from(raw: usize) -> Button {
        match raw {
            1 => Button::Left,
            2 => Button::Middle,
            3 => Button::Right,
            4 => Button::ScrollUp,
            5 => Button::ScrollDown,
            i => Button::Other(i),
        }
    }
}
