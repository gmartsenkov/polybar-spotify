use dbus::{blocking::Connection, arg};
use std::collections::HashMap;
use std::time::Duration;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;

    let proxy = conn.with_proxy("org.mpris.MediaPlayer2.spotify", "/org/mpris/MediaPlayer2", Duration::from_millis(5000));

    let metadata: HashMap<String, arg::Variant<Box<dyn arg::RefArg>>> = match proxy.get("org.mpris.MediaPlayer2.Player", "Metadata") {
        Err(_) => {
            return Ok(())
        },
        Ok(val) => val
    };

    let album = metadata.get("xesam:album").expect("Error getting the album");
    let artist = metadata.get("xesam:artist").expect("Error getting the artist");
    let title = metadata.get("xesam:title").expect("Error getting the title");
    let play_status: Box<dyn arg::RefArg> = proxy.get("org.mpris.MediaPlayer2.Player", "PlaybackStatus")?;

    // Cast properties
    let album: &String = arg::cast(&album.0).unwrap();
    let title: &String = arg::cast(&title.0).unwrap();
    let artist: &Vec<String> = arg::cast(&artist.0).unwrap();
    let play_status: &String = arg::cast(&play_status).unwrap();

    println!("ALBUM IS {:?}", album);
    println!("Artist IS {:?}", artist.join(""));
    println!("STATUS IS {:?}", play_status);
    println!("TITLE IS {:?}", title);

    for (key, value) in metadata.iter() {
        println!("KEY {}, VALUE {:?} ", key, &value);
    }

    Ok(())
}
