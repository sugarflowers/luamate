mod clipboard;
mod dialog;
mod file;
mod json;
mod regex;
mod terminal;
mod text;

use clap::Parser;
use mlua::{Lua, Variadic};
//use anyhow::Result;
use colored::{Colorize, control};

#[derive(Parser)]
struct Args {
    /// 実行する Lua スクリプトのパス
    script: String,
}

fn print_lua_error(err: mlua::Error) {
    match err {
        mlua::Error::CallbackError { traceback, cause } => {
            println!("{}", format!("Error! {}", cause).red().bold());

            println!("{}", format!("{}", traceback).red().bold());
        }

        _ => {
            println!("{}", format!("{}", err).red().bold());
        }
    }
}

fn main() -> mlua::Result<()> {
    let args = Args::parse();
    let lua = Lua::new();

    // init colored console
    if cfg!(target_os = "windows") {
        control::set_virtual_terminal(true)
            .map_err(|_| mlua::Error::external("failed to enable virtual terminal"))?;
    }

    let print_fn = lua.create_function(|_, args: Variadic<String>| {
        for arg in args {
            println!("{}", arg);
        }
        Ok(())
    })?;

    let input_fn = lua.create_function(|_, ()| {
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s)
            .map_err(mlua::Error::external)?;
        Ok(s.trim_end().to_string())
    })?;

    lua.globals().set("print", print_fn)?;
    lua.globals().set("input", input_fn)?;

    let file = lua.create_table()?;
    let json = lua.create_table()?;
    let regex = lua.create_table()?;
    let text = lua.create_table()?;
    let terminal = lua.create_table()?;
    let clipboard = lua.create_table()?;
    let dialog = lua.create_table()?;

    file::register_file(&lua, &file)?;
    json::register_json(&lua, &json)?;
    regex::register_regex(&lua, &regex)?;
    text::register_text(&lua, &text)?;
    terminal::register_terminal(&lua, &terminal)?;
    clipboard::register_clipboard(&lua, &clipboard)?;
    dialog::register_dialog(&lua, &dialog)?;

    lua.globals().set("file", file)?;
    lua.globals().set("json", json)?;
    lua.globals().set("regex", regex)?;
    lua.globals().set("text", text)?;
    lua.globals().set("terminal", terminal)?;
    lua.globals().set("clipboard", clipboard)?;
    lua.globals().set("dialog", dialog)?;

    let script = file::read_text_auto(&args.script).map_err(mlua::Error::external)?;
    if let Err(e) = lua.load(&script).set_name(&args.script).exec() {
        print_lua_error(e);
    }

    Ok(())
}
