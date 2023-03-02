# ircqrs
single-binary irc quote website

## environment variables
- `IRCQRS_BIND` - set address for ircqrs to listen on, defaults to
  `127.0.0.1:8326`

## (re)building
ircqrs will include the `quotes/` directory when compiling.
inside, quotes should be stored as plain text files with
names in the format of `<number>.txt` where `<number>` is an
integer quote id with no leading zeros.

the compiler tries to be smart and will not rebuild stuff if
it thinks the inputs were not modified. however, it only
checks existing files, not new ones, so adding a quote will
not let it rebuild.

touch one of the src files to avoid this and force it to
rebuild:

```sh
touch src/main.rs
```

