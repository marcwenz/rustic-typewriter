mod input;
mod typing;
mod utils;
mod words;

use unicode_width::UnicodeWidthStr; // To get the width of some text.

use std::char;

use cursive::{
    default,
    event::{Event, EventResult, Key},
    theme::{Color, ColorStyle, Palette, Theme},
    utils::span::SpannedString,
    views::{Canvas, Dialog, LinearLayout, TextView},
};

fn main() {
    let mut siv = default();

    siv.add_layer(make_input());

    siv.add_global_callback(Key::Enter, |s| s.quit());

    siv.set_theme(make_theme());

    siv.run();
}

// fn build_linear_layout(words: TextView, input: Canvas<String>) -> LinearLayout {}

fn build_words_list() -> Canvas<String> {
    // Build a canvas around a string using terminal colors
    //
    let c = char::from_u32(0x0020).unwrap();
    let state = String::new();
    let canvas = Canvas::new(state)
        .with_draw(|text: &String, printer| {
            // Simply print our string
            printer.with_color(make_color_style(), |printer| {
                printer.print((0, 0), text);
            });
        })
        .with_on_event(|text: &mut String, event| match event {
            Event::Char(c) => {
                // Callback to check correctness
                if c.to_string() == String::from(" ") {
                    // Callback to advance word
                    text.clear();
                } else {
                    text.push(c);
                }
                EventResult::Consumed(None)
            }
            Event::Key(Key::Enter) => {
                let text = text.clone();
                EventResult::with_cb(move |s| {
                    s.add_layer(Dialog::info(&text));
                })
            }
            _ => EventResult::Ignored,
        })
        .with_required_size(|text, _constraints| (text.width(), 1).into());

    canvas
}

fn make_input() -> Canvas<String> {
    // Build a canvas around a string using terminal colors
    //
    let c = char::from_u32(0x0020).unwrap();
    let state = String::new();
    let canvas = Canvas::new(state)
        .with_draw(|text: &String, printer| {
            // Simply print our string
            printer.with_color(make_color_style(), |printer| {
                printer.print((0, 0), text);
            });
        })
        .with_on_event(|text: &mut String, event| match event {
            Event::Char(c) => {
                // Callback to check correctness
                if c.to_string() == String::from(" ") {
                    // Callback to advance word
                    text.clear();
                } else {
                    text.push(c);
                }
                EventResult::Consumed(None)
            }
            Event::Key(Key::Enter) => {
                let text = text.clone();
                EventResult::with_cb(move |s| {
                    s.add_layer(Dialog::info(&text));
                })
            }
            _ => EventResult::Ignored,
        })
        .with_required_size(|text, _constraints| (text.width(), 1).into());

    canvas
}

fn make_theme() -> Theme {
    let mut theme = Theme::default();
    let mut palette = Palette::default();
    palette.set_color("background", Color::TerminalDefault);

    theme.palette = palette;
    theme.shadow = false;

    theme
}

fn make_color_style() -> ColorStyle {
    ColorStyle::terminal_default()
}
