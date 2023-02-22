<div align="center">

# Zellij Bare Bar

A very simple tab bar plugin for [zellij](https://github.com/zellij-org/zellij)

![](https://github.com/tarkah/zellij-bare-bar/blob/master/assets/demo.gif)

</div>

## Installation

Plugins location is different for each platform, you can run `zellij setup --dump-plugins` 
to see where it dumps the built-in plugins.

Build and copy plugin to zellij plugins folder. 

```sh
rustup target add wasm32-wasi

cargo build --release

cp ./target/wasm32-wasi/release/bare-bar.wasm $HOME/.local/share/zellij/plugins/
```

Add plugin to one of your layouts. Path must be absolute, there seems to be a bug 
preventing relative path plugin name.

```kdl
layout {
    pane
    pane size=1 borderless=true {
        plugin location="file:///home/<user>/.local/share/zellij/plugins/bare-bar.wasm"
    }
}
```
