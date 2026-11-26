mod audio;
mod clipboard;
mod dialog;
mod file;
mod json;
mod terminal;
mod text;

use clap::Parser;
use mlua::{Lua, Variadic, Table};
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

pub fn load_builtin(lua: &Lua) -> mlua::Result<()> {
    let script = include_str!("../builtin/string.lua");
    lua.load(script).exec()?;
    Ok(())
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
    lua.globals().set("print", print_fn)?;

    
    let file = lua.create_table()?;
    let json = lua.create_table()?;
    let terminal = lua.create_table()?;
    let clipboard = lua.create_table()?;
    let dialog = lua.create_table()?;
    let audio = lua.create_table()?;
    //let string_tbl: Table = lua.globals().get("string")?;

    file::register_file(&lua, &file)?;
    json::register_json(&lua, &json)?;
    //text::register_text(&lua, &string_tbl)?;
    terminal::register_terminal(&lua, &terminal)?;
    clipboard::register_clipboard(&lua, &clipboard)?;
    dialog::register_dialog(&lua, &dialog)?;
    audio::register_audio(&lua, &audio)?;

    lua.globals().set("file", file)?;
    lua.globals().set("json", json)?;
    lua.globals().set("terminal", terminal)?;
    lua.globals().set("clipboard", clipboard)?;
    lua.globals().set("dialog", dialog)?;
    lua.globals().set("audio", audio)?;


    load_builtin(&lua)?;

    let script = file::read_text_auto(&args.script).map_err(mlua::Error::external)?;
    if let Err(e) = lua.load(&script).set_name(&args.script).exec() {
        print_lua_error(e);
    }

    Ok(())
}
