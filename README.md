# ezexfil

An easy alternative for exfiltrating information on target machines, similar to other common techniques such as a Discord WebHook.

Ezexfil will generate a log file every time it is ran and will store all data POSTed to it. This can be interchanged with a Discord WebHook URL which will allow it to work in the same exact way.

The main benefits of this tool over WebHooks are that Discord cannot track data being sent, and there is no enforcement of source origin (which is true for Discord WebHooks, which cannot recieve requests from origin discord.com)

# Build

Like any Rust crate, build with `cargo build --release`. Alternatively, you can use `cargo run` to run without building a binary, but this is not really recommended.

# Disclaimer

This is a tool built for fun. I am not responsible for anyone using this tool for malicious purposes. Similar tools exist and they are not hard to build yourself! Have fun!
