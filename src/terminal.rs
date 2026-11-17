use colored::{Color, Colorize, Style, Styles, control, ColoredString};
use mlua::{Lua, Table};

fn parse_color(name: &str) -> Color {
    match name.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "bright_black" => Color::BrightBlack,
        "bright_red" => Color::BrightRed,
        "bright_yellow" => Color::BrightYellow,
        "bright_blue" => Color::BrightBlue,
        "bright_magenta" => Color::BrightMagenta,
        "bright_cyan" => Color::BrightCyan,
        "birhgt_white" => Color::BrightWhite,
        _ => Color::White,
    }
}

fn apply_style(text: ColoredString, style: &str) -> ColoredString {
    match style.to_lowercase().as_str() {
        "bold" => text.bold(),
        "italic" => text.italic(),
        "underline" => text.underline(),
        "dimmed" => text.dimmed(),
        "blink" => text.blink(),
        "reversed" => text.reversed(),
        "hidden" => text.hidden(),
        "strikethrough" => text.strikethrough(),
        _ => text,
    }
}

pub fn register_terminal(lua: &Lua, tbl: &Table) -> mlua::Result<()> {
    let color_fn = lua.create_function(
        |_, (text, fg, bg, styles): (String, Option<String>, Option<String>, Option<Table>)| {
            let mut styled = text.normal();

            if let Some(color) = fg {
                styled = styled.color(parse_color(&color));
            }

            if let Some(color) = bg {
                styled = styled.on_color(parse_color(&color));
            }

            if let Some(styles) = styles {
                for style in styles.sequence_values::<String>() {
                    let style = style?;

                    styled = apply_style(styled, &style);
                }
            }

            Ok(styled.to_string())
        },
    )?;
    tbl.set("color", color_fn)?;

    Ok(())
}
