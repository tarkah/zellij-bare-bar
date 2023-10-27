use std::collections::BTreeMap;

use unicode_width::UnicodeWidthStr;
use zellij_tile::prelude::*;
use zellij_tile_utils::style;

#[derive(Debug, Default)]
pub struct Segment {
    text: String,
    fg: PaletteColor,
    bg: PaletteColor,
}

#[derive(Default)]
struct State {
    tabs: Vec<TabInfo>,
    mode_info: ModeInfo,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        set_selectable(false);
        subscribe(&[EventType::TabUpdate, EventType::ModeUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::ModeUpdate(mode_info) => {
                if self.mode_info != mode_info {
                    self.mode_info = mode_info;
                    return true;
                }
            }
            Event::TabUpdate(tabs) => {
                self.tabs = tabs;
                return true;
            }
            _ => {
                eprintln!("Got unrecognized event: {:?}", event);
            }
        }
        false
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        if self.tabs.is_empty() {
            return;
        }

        let mode = self.mode_info.mode;
        let palette = self.mode_info.style.colors;

        let tabs = self.tabs.iter().map(|tab| tab_segment(tab, mode, palette));
        let line = line_segments(cols, tabs, mode, palette);

        print(line, palette);
    }
}

fn tab_segment(tab: &TabInfo, mode: InputMode, palette: Palette) -> Segment {
    let is_default_name = |tab: &TabInfo| tab.name.starts_with("Tab #");

    let text = if tab.active && tab.name.is_empty() && matches!(mode, InputMode::RenameTab) {
        "..."
    } else if is_default_name(tab) {
        "_"
    } else {
        &tab.name
    };

    let formatted = format!("{}:{text} ", tab.position + 1);

    let fg = if tab.active { palette.blue } else { palette.fg };

    Segment {
        text: formatted,
        fg,
        bg: palette.black,
    }
}

fn prefix(mode: InputMode, palette: Palette) -> Segment {
    const DOT: char = '•';

    let fg = match mode {
        InputMode::Locked => palette.magenta,
        InputMode::Normal => palette.green,
        _ => palette.orange,
    };

    Segment {
        text: format!(" {DOT} "),
        fg,
        bg: palette.black,
    }
}

fn line_segments(
    cols: usize,
    tabs: impl Iterator<Item = Segment>,
    mode: InputMode,
    palette: Palette,
) -> impl Iterator<Item = Segment> {
    vec![prefix(mode, palette)]
        .into_iter()
        .chain(tabs)
        .scan(cols as isize, |remaining, segment| {
            *remaining -= segment.text.width() as isize;

            (*remaining >= 0).then_some(segment)
        })
}

fn print(segments: impl Iterator<Item = Segment>, palette: Palette) {
    let render_segment = |segment: Segment| {
        style!(segment.fg, segment.bg)
            .paint(segment.text)
            .to_string()
    };

    let rendered = segments.map(render_segment).collect::<String>();

    match palette.black {
        PaletteColor::Rgb((r, g, b)) => {
            print!("{}\u{1b}[48;2;{};{};{}m\u{1b}[0K", rendered, r, g, b);
        }
        PaletteColor::EightBit(color) => {
            print!("{}\u{1b}[48;5;{}m\u{1b}[0K", rendered, color);
        }
    }
}
