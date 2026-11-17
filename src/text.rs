use mlua::{Lua, Table};

pub fn register_text(lua: &Lua, tbl: &Table) -> mlua::Result<()> {
    let trim_fn = lua.create_function(|_, value: String| Ok(value.trim().to_string()))?;
    tbl.set("trim", trim_fn)?;

    let split_fn = lua.create_function(|lua, (text, delim): (String, String)| {
        let table = lua.create_table()?;
        for (i, part) in text.split(&delim).enumerate() {
            table.set(i + 1, part)?;
        }
        Ok(table)
    })?;
    tbl.set("split", split_fn)?;

    let join_fn = lua.create_function(|_, (table, delim): (Table, String)| {
        let mut items = Vec::new();
        for value in table.sequence_values::<String>() {
            items.push(value?);
        }
        Ok(items.join(&delim))
    })?;
    tbl.set("join", join_fn)?;

    Ok(())
}
