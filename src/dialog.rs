use mlua::{Lua, Table};
use rfd::{MessageDialog, MessageButtons, MessageDialogResult, MessageLevel};

fn parse_level(level: &str) -> MessageLevel {
    match level.to_lowercase().as_str() {
        "info" => MessageLevel::Info,
        "warning" => MessageLevel::Warning,
        "error" => MessageLevel::Error,
        _ => MessageLevel::Info,
    }
}

fn disp_dialog(title: &str, description: &str, level: Option<String>, button: MessageButtons) -> MessageDialogResult {
    let mut msg_level = MessageLevel::Info;
    if let Some(lv) = level {
        msg_level = parse_level(&lv)
    }

    let re = MessageDialog::new()
        .set_title(title)
        .set_description(description)
        .set_level(msg_level)
        .set_buttons(button)
        .show();

    re
}

pub fn register_dialog(lua: &Lua, tbl: &Table) -> mlua::Result<()> {

    let msgbox_fn = lua.create_function(|_, (title, description, level): (String, String, Option<String>)| {
        disp_dialog(&title, &description, level, MessageButtons::Ok);
        Ok(())
    })?;
    tbl.set("msgbox", msgbox_fn)?;

    
    let yes_no_fn = lua.create_function(|_, (title, description, level): (String, String, Option<String>)| {
        let re = disp_dialog(&title, &description, level, MessageButtons::YesNo);
        Ok(re == MessageDialogResult::Yes)
    })?;
    tbl.set("yes_no", yes_no_fn)?;

    Ok(())
}
