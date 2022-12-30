# ircqrs
irc quote website thingy

## rebuilding

the compiler tries to be smart and will not rebuild stuff if
it thinks the inputs were not modified. however, it only
checks existing files, not new ones, so adding a quote will
not let it rebuild.

touch one of the src files to avoid this and force it to
rebuild.

```sh
touch src/main.rs
```

