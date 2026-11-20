use anyhow::Result;
use mlua::{Lua, Table};
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

fn parse_level(level: &str) -> MessageLevel {
    match level.to_lowercase().as_str() {
        "info" => MessageLevel::Info,
        "warning" => MessageLevel::Warning,
        "error" => MessageLevel::Error,
        _ => MessageLevel::Info,
    }
}

fn disp_dialog(
    title: Option<String>,
    description: &str,
    level: Option<String>,
    button: MessageButtons,
) -> MessageDialogResult {
    let mut msg_level = MessageLevel::Info;
    if let Some(lv) = level {
        msg_level = parse_level(&lv)
    }
    let mut title_str = "Message".to_string();
    if let Some(titl) = title {
        title_str = titl
    }
    let re = MessageDialog::new()
        .set_title(title_str)
        .set_description(description)
        .set_level(msg_level)
        .set_buttons(button)
        .show();
    re
}

fn create_file_dialog(
    title: Option<String>,
    filters: Option<Table>,
    dir: Option<String>,
) -> Result<FileDialog> {
    let mut dialog = FileDialog::new();
    if let Some(filters) = filters {
        let mut exts = Vec::<String>::new();
        for ext in filters.sequence_values::<String>() {
            exts.push(ext.map_err(|e| anyhow::anyhow!("{}", e))?);
        }
        let refs: Vec<&str> = exts.iter().map(String::as_str).collect();
        dialog = dialog.add_filter("files", &refs);
    }
    if let Some(t) = title {
        dialog = dialog.set_title(&t);
    }
    if let Some(d) = dir {
        //It is not working properly.
        dialog = dialog.set_directory(&d);
    }
    Ok(dialog)
}

fn disp_file_dialog(
    title: Option<String>,
    filters: Option<Table>,
    dir: Option<String>,
) -> Result<Option<String>> {
    let dialog = create_file_dialog(title, filters, dir);
    Ok(dialog?
        .pick_file()
        .map(|p| p.to_string_lossy().replace("\\", "/")))
}

fn disp_files_dialog(
    title: Option<String>,
    filters: Option<Table>,
    dir: Option<String>,
) -> Result<Option<Vec<String>>> {
    let dialog = create_file_dialog(title, filters, dir);
    Ok(dialog?.pick_files().map(|paths| {
        paths
            .into_iter()
            .map(|p| p.to_string_lossy().replace("\\", "/"))
            .collect::<Vec<String>>()
    }))
}

fn disp_save_dialog(
    file_name: Option<String>,
    title: Option<String>,
    filters: Option<Table>,
    dir: Option<String>,
) -> Result<Option<String>> {
    let mut dialog = create_file_dialog(title, filters, dir)?;
    if let Some(filename) = file_name {
        dialog = dialog.set_file_name(&filename);
    }
    Ok(dialog
        .save_file()
        .map(|p| p.to_string_lossy().replace("\\", "/")))
}



fn disp_dir_dialog(
    title: Option<String>,
    dir: Option<String>,
) -> Result<Option<String>> {
    let dialog = create_file_dialog(title, None, dir);
    Ok(dialog?
        .pick_folder()
        .map(|p| p.to_string_lossy().replace("\\", "/")))
}


fn disp_dirs_dialog(
    title: Option<String>,
    dir: Option<String>,
) -> Result<Option<Vec<String>>> {
    let dialog = create_file_dialog(title, None, dir);
    Ok(dialog?.pick_folders().map(|paths| {
        paths
            .into_iter()
            .map(|p| p.to_string_lossy().replace("\\", "/"))
            .collect::<Vec<String>>()
    }))
}


pub fn register_dialog(lua: &Lua, tbl: &Table) -> mlua::Result<()> {
    let msgbox_fn = lua.create_function(
        |_, (description, title, level): (String, Option<String>, Option<String>)| {
            disp_dialog(title, &description, level, MessageButtons::Ok);
            Ok(())
        },
    )?;
    tbl.set("msgbox", msgbox_fn)?;

    let yes_no_fn = lua.create_function(
        |_, (description, title, level): (String, Option<String>, Option<String>)| {
            let re = disp_dialog(title, &description, level, MessageButtons::YesNo);
            Ok(re == MessageDialogResult::Yes)
        },
    )?;
    tbl.set("yes_no", yes_no_fn)?;

    let select_file_fn = lua.create_function(
        |_, (title, filters, dir): (Option<String>, Option<Table>, Option<String>)| {
            let path = disp_file_dialog(title, filters, dir).map_err(mlua::Error::external)?;
            Ok(path)
        },
    )?;
    tbl.set("select_file", select_file_fn)?;

    let select_files_fn = lua.create_function(
        |_, (title, filters, dir): (Option<String>, Option<Table>, Option<String>)| {
            let path = disp_files_dialog(title, filters, dir).map_err(mlua::Error::external)?;
            Ok(path)
        },
    )?;
    tbl.set("select_files", select_files_fn)?;

    let select_save_fn = lua.create_function(
        |_, (filename, title, filters, dir): (Option<String>, Option<String>, Option<Table>, Option<String>)| {
            let path = disp_save_dialog(filename, title, filters, dir).map_err(mlua::Error::external)?;
            Ok(path)
        },
    )?;
    tbl.set("select_save", select_save_fn)?;

    let select_dir_fn = lua.create_function(
        |_, (title, dir): (Option<String>, Option<String>)| {
            let path = disp_dir_dialog(title, None).map_err(mlua::Error::external)?;
            Ok(path)
        },
    )?;
    tbl.set("select_dir", select_dir_fn)?;

    let select_dirs_fn = lua.create_function(
        |_, (title, dir): (Option<String>, Option<String>)| {
            let path = disp_dirs_dialog(title, None).map_err(mlua::Error::external)?;
            Ok(path)
        },
    )?;
    tbl.set("select_dirs", select_dirs_fn)?;

    Ok(())
}
