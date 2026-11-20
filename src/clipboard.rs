use arboard::Clipboard;
use mlua::{Lua, Table};

pub fn register_clipboard(lua: &Lua, tbl: &Table) -> mlua::Result<()> {
    let get_fn = lua.create_function(|_, (): ()| {
        let mut clipboard = Clipboard::new().map_err(mlua::Error::external)?;
        Ok(clipboard.get_text().map_err(mlua::Error::external)?)
    })?;
    tbl.set("get", get_fn)?;

    let set_fn = lua.create_function(|_, text: String| {
        let mut clipboard = Clipboard::new().map_err(mlua::Error::external)?;
        Ok(clipboard.set_text(&text).map_err(mlua::Error::external)?)
    })?;
    tbl.set("set", set_fn)?;

    Ok(())
}
