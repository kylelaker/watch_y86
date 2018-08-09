# watch_y86

A super simple rust program that uses the inotify crate to monitor one
hardcoded directory. Someday this might do more.

This is currently running as a simple systemd service and I check the
journal infrequently to see if it noticed any changes.
