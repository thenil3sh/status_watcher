Status Watcher
## What it does?
- `status_watcher` relies on DBus to listen for any watch request.
- `add_watcher`, lets say you want to know if `example.com` is live or not, you'll pass with `add_watcher`, then, `status_watcher` queues the request, and keeps pinging (every 30 secs), until it gets a good response.\
\
This is just a practice ~_project_~, to familiarize with rust async and DBus.
