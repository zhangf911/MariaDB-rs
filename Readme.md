Rust bindings and wrapper for MariaDB / MySQL.

This allows you to avoid using unsafe blocks for the FFI to C-functions,
while also avoiding using C-style strings.  The Connection wrapper also
has helper functions for queries.


Please open an issue (or better yet, send a PR!) if something is missing.


Current goals:
* Wrap as much functionality into a Rusty way.
* Wrap as much unsafe code into the smallest parts possible (unsafe is needed for the C-style strings and FFI functions)
* I plan on building a tiny framework-ish thing around it, probably just limited to a trait for (de)serializing to SQL.


License:
---------
This is released under the same license as the MariaDB client library, which is lGPLv2.1

