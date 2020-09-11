# polybar-spotify

A small command line tool to print the currently playing track on Spotify. It's written in Rust and using `dbus` to communicate with Spotify.

### Settings
```ini
[module/spotify]
type = custom/script
interval = 1
format-prefix = " "
format = <label>
exec = polybar-spotify
format-underline = #1db954
```

### Options
The format can be specified with the `--format=` argument, e.g. `--format="{playStatus}: {title}"`.
Available variables are - `playStatus`, `title`, `artist`, `album` 

### Limitations
Spotify will not share the song info if it's playing on a different device. It's possible that it'll get fixed at some point.
