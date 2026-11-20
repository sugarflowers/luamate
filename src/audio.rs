use mlua::{Lua, Table};
use rodio::{Decoder, Source, Player};
use std::io::BufReader;


use std::thread;
use std::time::Duration;

/*
struct AudioPlayer {
    player: Player,
    file: std::fs::File,
}

impl AudioPlayer {
    fn open(path: &str) -> anyhow::Result<Self> {
        let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
        Ok(Self {
            player: Player::connect_new(stream_handle.mixer()),
            file: std::fs::File::open(path)?
        })
    }
    fn play(self) -> anyhow::Result<()> {
        self.player.append(rodio::Decoder::try_from(self.file)?);
        self.player.sleep_until_end();

        Ok(())
    }
}

pub fn audio_test() -> anyhow::Result<()> {

    let mut au = AudioPlayer::open("sample/ok2.wav")?;
    //au.play()?;
    Ok(())
}
*/

fn play_sink(path: &str) -> anyhow::Result<()> {
    Ok(())
}

pub fn register_audio(lua: &Lua, tbl: &Table) -> mlua::Result<()> {

    //audio_test().map_err(mlua::Error::external)?;

    //thread::sleep(Duration::from_millis(1500));


    let play_fn = lua.create_function(|_, file_path: String| {

        Ok(())
    })?;
    tbl.set("play", play_fn)?;

    let play_wait_fn = lua.create_function(|_, file_path: String| {
        let stream_handle = rodio::DeviceSinkBuilder::open_default_sink().map_err(mlua::Error::external)?;
        let player = Player::connect_new(stream_handle.mixer());

        let file = std::fs::File::open(file_path).map_err(mlua::Error::external)?;
        player.append(rodio::Decoder::try_from(file).map_err(mlua::Error::external)?);

        player.sleep_until_end();
        Ok(())
    })?;
    tbl.set("play_wait", play_wait_fn)?;

    Ok(())
}
