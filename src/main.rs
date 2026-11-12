mod file;
mod json;

use clap::Parser;
use mlua::{Lua, Variadic};
//use anyhow::Result;

#[derive(Parser)]
struct Args {
    /// 実行する Lua スクリプトのパス
    script: String,
}

fn print_lua_error(err: mlua::Error) {
    match err {
        mlua::Error::CallbackError { traceback, cause } => {
            println!("Error: {}", cause);

            println!("{}", traceback);
        }

        _ => {
            println!("{}", err);
        }
    }
}

fn main() -> mlua::Result<()> {
    let args = Args::parse();
    let lua = Lua::new();

    let print_fn = lua.create_function(|_, args: Variadic<String>| {
        for arg in args {
            println!("{}", arg);
        }
        Ok(())
    })?;

    lua.globals().set("print", print_fn)?;
    let utils = lua.create_table()?;
    let file = lua.create_table()?;
    let json = lua.create_table()?;

    file::register_file(&lua, &file)?;
    json::register_json(&lua, &json)?;

    lua.globals().set("utils", utils)?;
    lua.globals().set("file", file)?;
    lua.globals().set("json", json)?;

    let script = file::read_text_auto(&args.script).map_err(mlua::Error::external)?;
    if let Err(e) = lua.load(&script).set_name(&args.script).exec() {
        print_lua_error(e);
    }

    Ok(())
}
