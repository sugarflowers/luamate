use mlua::{Lua, Table, UserData, UserDataMethods};
use regex::Regex;

pub fn register_regex(lua: &Lua, tbl: &Table) -> mlua::Result<()> {
    let check_fn = lua.create_function(|_, (pattern, text): (String, String)| {
        let re = Regex::new(&pattern).map_err(mlua::Error::external)?;
        Ok(re.is_match(&text))
    })?;
    tbl.set("check", check_fn)?;

    let match_fn = lua.create_function(|lua, (pattern, text): (String, String)| {
        let re = regex::Regex::new(&pattern).map_err(mlua::Error::external)?;

        let mut results = mlua::MultiValue::new();

        if let Some(caps) = re.captures(&text) {
            for i in 1..caps.len() {
                let value = caps.get(i).map(|m| m.as_str()).unwrap_or("");

                results.push_back(lua.pack(value.to_string())?);
            }
        }

        Ok(results)
    })?;

    tbl.set("match", match_fn)?;

    Ok(())
}
