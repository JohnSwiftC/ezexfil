# ezexfil

An easy to use information exfiltration tool built with Rust, which logs all information sent via a http POST request.

You may also optionally forward logged data to a Discord WebHook. When doing this, just use the script you are already using a webhook in, and replace it with the machine hosting this tool. Then provide your Discord WebHook URL as the first argument when starting ezexfil.

This allows for two main things: your WebHook URL is shielded, and you can send requests with an original origin of `www.discord.com`, which is normally prohibited.

# Build

Like any Rust crate, build with `cargo build --release`. Alternatively, you can use `cargo run` to run without building a binary, but this is not really recommended.

# Usage

Ezexfil automatically logs all data sent via a POST request to a log file. Optionally, you can choose to proxy Discord WebHook traffic through the tool by providing the URL as the first argument.

Ex. ezexfil \<port\> \<Optional WebHook URL\>

It is important to note that if you use this option, data sent must be in valid Discord WebHook JSON already. The url hosting this tool can be plugged into the same place where you would put your
normal WebHook URL.

# Disclaimer

This is a tool built for fun. I am not responsible for anyone using this tool for malicious purposes. Similar tools exist and they are not hard to build yourself! Have fun!
