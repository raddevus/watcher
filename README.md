# watcher
File System Watcher implemented in Rust - can watch events on directories &amp; tail files (continuously watches files or directories)

version 0.1.0 

You can use The File System Watcher in two main ways.
1. You can watch the events on a directory -- supply a path with no file name.
2. Tail a file continuously -- supply a file name which will output contents to console each time file is written to*


*File functionality still isn't entirely complete -- there are output statements for testing it still being displayed.

USAGE

$ fwx /path/subdir  --- as files are altered in the directory, events will be displayed in console

$ fwx filename --- as the file is written to, the new contents will be displayed to screen
