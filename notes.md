## Notes

### Github Pages 404 errors

When first published this site threw 404 errors when it tried to load the wasm bundles and css files.
This is because github pages sees "*load /styles.css*" and tries to load them from `https://brstrutt.github.io/styles.css`.
Obviously this fails because the styles are ACTUALLY at `https://brstrutt.github.io/IsWorcesterUnderwaterV2/styles.css`.
Looks like one solution is to remove the opening `/` from the filepaths....but how can I get trunk to do that?
The solution seems to be to create a `Trunk.toml` file containing the following:
```toml
[build]
public_url = "/IsWorcesterUnderwaterV2/"
```
Now my release build has the specified `public_url` prefixed onto all the 404-ing links.
