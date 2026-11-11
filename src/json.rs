use mlua::{Lua, Table, Value};
use serde_json::{Map, Number, Value as JsonValue};

fn json_to_lua(lua: &Lua, value: JsonValue) -> mlua::Result<Value> {
    match value {
        JsonValue::Null => Ok(Value::Nil),

        JsonValue::Bool(v) => Ok(Value::Boolean(v)),

        JsonValue::Number(v) => {
            if let Some(i) = v.as_i64() {
                Ok(Value::Integer(i))
            } else if let Some(f) = v.as_f64() {
                Ok(Value::Number(f))
            } else {
                Ok(Value::Nil)
            }
        }

        JsonValue::String(v) => Ok(Value::String(lua.create_string(&v)?)),

        JsonValue::Array(arr) => {
            let table = lua.create_table()?;

            for (i, v) in arr.into_iter().enumerate() {
                table.set(i + 1, json_to_lua(lua, v)?)?;
            }

            Ok(Value::Table(table))
        }

        JsonValue::Object(obj) => {
            let table = lua.create_table()?;

            for (k, v) in obj {
                table.set(k, json_to_lua(lua, v)?)?;
            }

            Ok(Value::Table(table))
        }
    }
}

fn lua_to_json(value: Value) -> mlua::Result<JsonValue> {
    match value {
        Value::Nil => Ok(JsonValue::Null),

        Value::Boolean(v) => Ok(JsonValue::Bool(v)),

        Value::Integer(v) => Ok(JsonValue::Number(Number::from(v))),

        Value::Number(v) => Ok(JsonValue::Number(
            Number::from_f64(v).ok_or_else(|| mlua::Error::external("invalid number"))?,
        )),

        Value::String(v) => Ok(JsonValue::String(v.to_str()?.to_string())),

        Value::Table(table) => table_to_json(table),

        _ => Err(mlua::Error::external("unsupported value type")),
    }
}

fn table_to_json(table: Table) -> mlua::Result<JsonValue> {
    let len = table.raw_len();

    let mut is_array = true;
    let mut count = 0;

    for pair in table.pairs::<Value, Value>() {
        let (key, _) = pair?;

        count += 1;

        match key {
            Value::Integer(i) if i >= 1 && (i as usize) <= len => {}

            _ => {
                is_array = false;
                break;
            }
        }
    }

    if is_array && count == len {
        let mut array = Vec::new();

        for i in 1..=len {
            let value: Value = table.raw_get(i)?;
            array.push(lua_to_json(value)?);
        }

        Ok(JsonValue::Array(array))
    } else {
        let mut object = Map::new();

        for pair in table.pairs::<String, Value>() {
            let (key, value) = pair?;

            object.insert(key, lua_to_json(value)?);
        }

        Ok(JsonValue::Object(object))
    }
}

pub fn register_json(lua: &Lua, tbl: &Table) -> mlua::Result<()> {
    let encode_fn = lua.create_function(|_, value: Value| {
        let json = lua_to_json(value)?;
        serde_json::to_string_pretty(&json).map_err(mlua::Error::external)
    })?;
    tbl.set("encode", encode_fn)?;

    let decode_fn = lua.create_function(|lua, text: String| {
        let json: JsonValue = serde_json::from_str(&text).map_err(mlua::Error::external)?;

        json_to_lua(lua, json)
    })?;
    tbl.set("decode", decode_fn)?;

    Ok(())
}
