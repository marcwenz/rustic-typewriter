mod input;
mod typing;
mod utils;
mod words;

// let mut styled = StyledString::plain("Isn't ");
// styled.append(StyledString::styled("that ", Color::Dark(BaseColor::Red)));
// styled.append(StyledString::styled(
//     "cool?",
//     Style::from(Color::Light(BaseColor::Blue)).combine(Effect::Bold),
// ));
use unicode_width::UnicodeWidthStr; // To get the width of some text.

use std::char;

use cursive::{
    default,
    event::{Event, EventResult, Key},
    theme::{BaseColor, Color, ColorStyle, ColorType, Palette, PaletteColor, Theme},
    utils::markup::StyledString,
    views::{Canvas, Dialog, EditView, EnableableView, LinearLayout, TextView},
};

use crate::words::WordList;

fn main() {
    let mut siv = default();

    siv.set_theme(make_theme());
    let words_list = build_words_list();
    let input = build_input();
    let mut lin = build_linear_layout(words_list, input);
    let dd = TextView::new(lin.len().to_string());
    lin.add_child(dd);

    siv.add_layer(lin);

    siv.add_global_callback(Key::Enter, |s| s.quit());

    siv.run();
}

fn build_linear_layout(words: TextView, input: Canvas<EditView>) -> LinearLayout {
    let mut lin = LinearLayout::vertical();
    lin.add_child(words);
    lin.add_child(input);
    lin
}

fn build_words_list() -> TextView {
    let mut words = WordList::new(100);

    TextView::new(StyledString::styled(
        words.get_random_range(5).unwrap().join(" "),
        make_color_style(),
    ))
}

fn build_input() -> Canvas<EditView> {
    // Build a canvas around a string using terminal colors
    //
    let state = EditView::new();
    let canvas = Canvas::new(state)
        .with_draw(|text: &EditView, printer| {
            // Simply print our string
            printer.with_color(make_color_style_input(), |printer| {
                printer.print((0, 1), text);
            });
        })
        .with_on_event(|text: &mut EditView, event| match event {
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
        .with_required_size(|text, _constraints| (text.width(), 2).into());

    canvas
}

fn make_theme() -> Theme {
    let mut theme = Theme::default();
    let mut palette = Palette::default();
    palette.set_color("background", Color::TerminalDefault);
    palette.set_color("foreground", Color::Light(BaseColor::White));
    palette.set_color("primary", Color::Light(BaseColor::White));
    // palette.set_color("view", Color::Light(BaseColor::White));
    palette.set_color("view", Color::TerminalDefault);

    theme.palette = palette;
    theme.shadow = false;

    theme
}

fn make_color_style() -> ColorStyle {
    ColorStyle::terminal_default()
}

fn make_color_style_input() -> ColorStyle {
    let mut color = ColorStyle::terminal_default();

    color.back = ColorType::Color(Color::TerminalDefault);
    color
}
