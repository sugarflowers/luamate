# LuaMate
**Your automation mate.**
LuaMate is a Lua-based automation companion built with Rust.

It is designed to simplify everyday work automation by providing a lightweight scripting environment with powerful built-in capabilities. Instead of creating a new tool for every task, users can automate workflows using small Lua scripts running on a common runtime.

## Features

- Lightweight Lua scripting
- Fast and reliable Rust runtime
- Automatic UTF-8 / Shift-JIS text handling
- Streaming file processing
- Extensible module architecture
- Windows-focused workflow automation

## Current Capabilities

- Execute Lua scripts
- Read text files with automatic encoding detection
- Iterate through files line by line
- Create and append text files
- Shift-JIS text output
- Rust UserData integration

## Planned Features

- Directory operations
- CSV and JSON support
- Serial communication
- TCP/IP communication
- HTTP client support
- Excel file handling
- Audio playback
- External process control
- Label printer integration
- Device provisioning workflows

## Philosophy

LuaMate is not intended to be another RPA platform.

The goal is to provide a simple and reliable automation companion that allows users to describe workflows in Lua while delegating low-level operations to Rust.

A typical LuaMate script should focus on business logic rather than technical implementation details.