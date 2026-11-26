use mlua::{Lua, Table};
use rodio::Player;
use std::io::BufReader;

fn play_async(file_path: String) {
    std::thread::spawn(move || {
        let mut stream_handle = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
        stream_handle.log_on_drop(false);
        let mixer = stream_handle.mixer();
        let file = std::fs::File::open(&file_path).unwrap();
        let sink = rodio::play(mixer, BufReader::new(file)).unwrap();

        sink.sleep_until_end();
    });
}

pub fn register_audio(lua: &Lua, tbl: &Table) -> mlua::Result<()> {
    let play_fn = lua.create_function(|_, path: String| {
        let file = std::fs::File::open(&path).map_err(mlua::Error::external)?;
        drop(file);
        play_async(path);
        Ok(())
    })?;
    tbl.set("play", play_fn)?;

    let play_wait_fn = lua.create_function(|_, file_path: String| {
        let mut stream_handle =
            rodio::DeviceSinkBuilder::open_default_sink().map_err(mlua::Error::external)?;
        stream_handle.log_on_drop(false);
        let player = Player::connect_new(stream_handle.mixer());
        let file = std::fs::File::open(file_path).map_err(mlua::Error::external)?;
        player.append(rodio::Decoder::try_from(file).map_err(mlua::Error::external)?);
        player.sleep_until_end();
        Ok(())
    })?;
    tbl.set("play_wait", play_wait_fn)?;

    Ok(())
}
