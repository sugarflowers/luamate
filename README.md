# LuaMate

**Your automation mate.**

LuaMate is a Lua scripting environment built with Rust and mlua to simplify everyday automation tasks.

## Features

- Lua scripting powered by Rust and mlua
- Automatic UTF-8 / Shift-JIS text handling
- Practical modules for everyday automation
- Simple and lightweight runtime

## Current Capabilities

- Read and write text files with automatic encoding handling
- Append text files
- Stream files line by line
- JSON string ↔ object conversion
- `string.split()`
- `string.trim()`
- Clipboard text get/set
- Colored terminal output
- Message dialogs
- File open dialogs
- Multiple file selection dialogs
- File save dialogs
- Asynchronous audio playback
- Synchronous audio playback

## Planned Features

- Serial communication
- Telnet client
- SSH client
- CSV processing
- SQLite support
- ZIP archive support
- Desktop automation
- Mouse and keyboard control
- Window operations
- Tepra label printer support

## Philosophy

LuaMate aims to keep automation scripts simple.

Workflows are written in Lua, while platform-specific and low-level functionality is handled by Rust.



## Example

```lua
local path = dialog.select_file(
  "Select a file",
  {"txt", "csv"}
)

local text = file.read(path)
clipboard.set(text)

colored = terminal.color("Copied file contents to clipboard.", "yellow", nil, {"bold"})
print(colored)
```

