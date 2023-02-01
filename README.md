# include_uri

Proc-macros that mimic std [include_str!](https://doc.rust-lang.org/std/macro.include_str.html) and [include_bytes!](https://doc.rust-lang.org/std/macro.include_bytes.html) but allows including remote files.

It's importanto to keep in mind it's _*dangerous*_ to include remote files from untrusted sources.
