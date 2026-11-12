use anyhow::Result;
use encoding_rs::SHIFT_JIS;
use glob::glob;
use mlua::{Lua, Table, UserData, UserDataMethods};
use std::{
    cell::RefCell,
    env, fs,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
    path::Path,
    rc::Rc,
};

#[derive(Debug, Clone, Copy)]
enum TextEncoding {
    Utf8,
    Utf8Bom,
    ShiftJis,
}

struct TextWriter {
    file: File,
}

impl TextWriter {
    fn create(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            file: File::create(path)?,
        })
    }
    fn open(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            file: OpenOptions::new().create(true).append(true).open(path)?,
        })
    }
}

impl UserData for TextWriter {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("write", |_, this, text: String| {
            let (encoded, _, _) = SHIFT_JIS.encode(&text);
            this.file
                .write_all(&encoded)
                .map_err(mlua::Error::external)?;
            Ok(())
        });

        methods.add_method_mut("writeln", |_, this, text: String| {
            let (encoded, _, _) = SHIFT_JIS.encode(&text);
            this.file
                .write_all(&encoded)
                .map_err(mlua::Error::external)?;
            this.file
                .write_all(b"\r\n")
                .map_err(mlua::Error::external)?;
            Ok(())
        });

        methods.add_method_mut("close", |_, this, ()| {
            this.file.flush().map_err(mlua::Error::external)?;
            Ok(())
        });
    }
}

struct LineReader {
    reader: BufReader<File>,
    encoding: TextEncoding,
}

impl LineReader {
    fn open(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;
        let encoding = detect_encoding(&mut file)?;
        match encoding {
            TextEncoding::Utf8Bom => {
                file.seek(SeekFrom::Start(3))?;
            }
            _ => {}
        }
        Ok(Self {
            reader: BufReader::new(file),
            encoding,
        })
    }
}
impl LineReader {
    fn next_line(&mut self) -> anyhow::Result<Option<String>> {
        let mut buf = Vec::new();

        let size = self.reader.read_until(b'\n', &mut buf)?;

        if size == 0 {
            return Ok(None);
        }

        let text = match self.encoding {
            TextEncoding::Utf8 => String::from_utf8_lossy(&buf).into_owned(),

            TextEncoding::Utf8Bom => String::from_utf8_lossy(&buf).into_owned(),

            TextEncoding::ShiftJis => {
                let (s, _, _) = SHIFT_JIS.decode(&buf);

                s.into_owned()
            }
        };

        Ok(Some(
            text.trim_end_matches('\n')
                .trim_end_matches('\r')
                .to_string(),
        ))
    }
}

fn detect_encoding(file: &mut std::fs::File) -> anyhow::Result<TextEncoding> {
    let mut buf = [0u8; 4096];
    let size = file.read(&mut buf)?;
    file.seek(SeekFrom::Start(0))?;
    let bytes = &buf[..size];
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Ok(TextEncoding::Utf8Bom);
    }
    if std::str::from_utf8(bytes).is_ok() {
        return Ok(TextEncoding::Utf8);
    }
    Ok(TextEncoding::ShiftJis)
}

fn decode_text(bytes: &[u8], encoding: TextEncoding) -> String {
    match encoding {
        TextEncoding::Utf8 => String::from_utf8_lossy(bytes).into_owned(),
        TextEncoding::Utf8Bom => String::from_utf8_lossy(&bytes[3..]).into_owned(),
        TextEncoding::ShiftJis => {
            let (text, _, _) = SHIFT_JIS.decode(bytes);
            text.into_owned()
        }
    }
}

pub fn read_text_auto(path: &str) -> anyhow::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let enc = detect_encoding(&mut file)?;
    let bytes = fs::read(path)?;
    Ok(decode_text(&bytes, enc))
}

pub fn register_file(lua: &Lua, tbl: &Table) -> mlua::Result<()> {
    let lines_fn = lua.create_function(|lua, path: String| {
        let reader = Rc::new(RefCell::new(
            LineReader::open(&path).map_err(mlua::Error::external)?,
        ));

        let iter_reader = reader.clone();

        let iter = lua.create_function_mut(move |_, ()| {
            let mut reader = iter_reader.borrow_mut();

            reader.next_line().map_err(mlua::Error::external)
        })?;

        Ok(iter)
    })?;
    tbl.set("lines", lines_fn)?;

    let create_file_fn = lua.create_function(|_, path: String| {
        TextWriter::create(&path).map_err(mlua::Error::external)
    })?;
    tbl.set("create_file", create_file_fn)?;

    let open_file_fn = lua.create_function(|_, path: String| {
        TextWriter::open(&path).map_err(mlua::Error::external)
    })?;
    tbl.set("open_file", open_file_fn)?;

    let read_fn = lua.create_function(|_, path: String| {
        Ok(read_text_auto(&path).map_err(mlua::Error::external))
    })?;
    tbl.set("read", read_fn)?;

    let exists_fn = lua.create_function(|_, path: String| Ok(Path::new(&path).exists()))?;
    tbl.set("exists", exists_fn)?;

    let isfile_fn = lua.create_function(|_, path: String| Ok(Path::new(&path).is_file()))?;
    tbl.set("isfile", isfile_fn)?;

    let makedirs_fn = lua.create_function(|_, path: String| {
        fs::create_dir_all(Path::new(&path))?;
        Ok(())
    })?;
    tbl.set("makedirs", makedirs_fn)?;

    let rmdir_fn = lua.create_function(|_, path: String| {
        fs::remove_dir_all(Path::new(&path))?;
        Ok(())
    })?;
    tbl.set("rmdir", rmdir_fn)?;

    let remove_fn = lua.create_function(|_, path: String| {
        fs::remove_file(Path::new(&path))?;
        Ok(())
    })?;
    tbl.set("remove", remove_fn)?;

    let copy_fn = lua.create_function(|_, (src, dst): (String, String)| {
        fs::copy(&src, &dst).map_err(mlua::Error::external)?;
        Ok(())
    })?;
    tbl.set("copy", copy_fn)?;

    let move_fn = lua.create_function(|_, (src, dst): (String, String)| {
        fs::rename(&src, &dst).map_err(mlua::Error::external)?;
        Ok(())
    })?;
    tbl.set("move", move_fn)?;

    let getcwd_fn = lua.create_function(|_, ()| {
        let path = env::current_dir()?;
        Ok(path.to_string_lossy().replace("\\", "/"))
    })?;
    tbl.set("getcwd", getcwd_fn)?;

    let exepath_fn = lua.create_function(|_, ()| {
        let mut path = env::current_exe()?;
        let _ = path.pop();
        Ok(path.to_string_lossy().replace("\\", "/"))
    })?;
    tbl.set("exepath", exepath_fn)?;

    let glob_fn = lua.create_function(|lua, pattern: String| {
        let mut paths = glob(&pattern).map_err(mlua::Error::external)?;
        let iter = lua.create_function_mut(move |_, ()| {
            loop {
                match paths.next() {
                    Some(Ok(path)) => {
                        return Ok(Some(path.to_string_lossy().replace("\\", "/")));
                        //return Ok(Some(path.to_string_lossy().to_string()));
                    }
                    Some(Err(_)) => {
                        continue;
                    }
                    None => {
                        return Ok(None::<String>);
                    }
                }
            }
        })?;
        Ok(iter)
    })?;
    tbl.set("glob", glob_fn)?;

    let basename_fn = lua.create_function(|_, path: String| {
        let name = std::path::Path::new(&path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        Ok(name)
    })?;
    tbl.set("basename", basename_fn)?;

    let dirname_fn = lua.create_function(|_, path: String| {
        let dir = std::path::Path::new(&path)
            .parent()
            .map(|p| {
                p.to_string_lossy().replace("\\", "/")
            })
        .unwrap_or_default();
        Ok(dir)
    })?;
    tbl.set("dirname", dirname_fn)?;


    Ok(())
}
