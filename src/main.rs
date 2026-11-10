mod file;

use mlua::{Lua, Variadic};
use clap::Parser;
//use anyhow::Result;

#[derive(Parser)]
struct Args {
    /// 実行する Lua スクリプトのパス
    script: String,
}


fn main() -> mlua::Result<()> {
    let args = Args::parse();
    let lua = Lua::new();

    let utils = lua.create_table()?;
    let file = lua.create_table()?;


    let print_fn = lua.create_function(|_, args: Variadic<String>| {
        for arg in args {
            println!("{}", arg);
        }
        Ok(())
    })?;
    lua.globals().set("print", print_fn)?;



    file::register_file(&lua, &file)?;

    lua.globals().set("utils", utils)?;
    lua.globals().set("file", file)?;


    let script = file::read_text_auto(&args.script).map_err(mlua::Error::external)?;
    lua.load(&script).exec()?;


    Ok(())
}
