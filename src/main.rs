extern crate inotify;

use std::path::PathBuf;

use inotify::{
    EventMask,
    WatchMask,
    Inotify,
};

/*
 * Simple tool for watching a specific directory. This is just going to run
 * as a systemd service and then log everything. I'll check the journal
 * every once in awhile. Eventually, I'll do something more interesting
 */

fn main() {
    let mut inotify = match Inotify::init() {
        Ok(i) => i,
        Err(error) => {
            panic!("Failed to initialize inotify: {:?}", error);
        },
    };
    let y86_dir_str = "/cs/students/cs261/y86";
    let y86_dir = PathBuf::from(y86_dir_str);
    let _watch = match inotify.add_watch(y86_dir,
                                         WatchMask::MODIFY
                                         | WatchMask::CREATE
                                         | WatchMask::DELETE) {
        Ok(watch) => watch,
        Err(error) => {
            panic!("Failed to watch the directory ({:?}): {:?}",
                   y86_dir_str, error)
        },
    };

    eprintln!("Watching {} for activity...", y86_dir_str);
    let mut buffer = [0u8; 4096];
    loop {
        let events = match inotify.read_events_blocking(&mut buffer) {
            Ok(e) => e,
            Err(error) => {
                eprintln!("Unable to read events: {:?}", error);
                continue
            },
        };

        for event in events {
            if event.mask.contains(EventMask::CREATE) {
                eprintln!("Created: {:?}", event.name);
            } else if event.mask.contains(EventMask::DELETE) {
                eprintln!("Deleted: {:?}", event.name);
            } else if event.mask.contains(EventMask::MODIFY) {
                eprintln!("Modified: {:?}", event.name);
            }
        }
    }
}
