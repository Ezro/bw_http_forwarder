A simple tool to forward the internal HTTP server in a Brood War client to a hardcoded port 3000.

## Running the HTTP Forwarder
1) Download the latest OS release asset
2) Extract the `bw_http_forwarder`
3) Run `chmod +x bw_http_forwarder` if needed
4) Start StarCraft
5) Run bw_http_forwarder

## Building the tool
1) Install [rustup](https://rustup.rs/)
2) Run `rustup update`
3) git clone bw_http_forwarder
4) `cd bw_http_forwarder`
5) `cargo build` or `cargo run`

## Testing the tool
While the tool is running, you should be able to make simple GET calls via your favorite tool (browser, postman, curl, etc.) with your StarCraft running.

Here is a snippet I captured using Fiddler while logging in and clicking around online:
![Fiddler snippet](https://raw.githubusercontent.com/Ezro/bw_http_forwarder/refs/heads/main/fiddler_snippet.png)

These calls appear to be directly available from the main menu as long as you're connected to Battle.net.

Enjoy.
