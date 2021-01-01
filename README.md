# xpoz

An API and a web gallery/viewer for your Apple's Photos.app albums.

I wrote a [blog post](1) explaining how some of that stuff works a while back.

## Building from source

Currently there are no prebuilt binaries. The only option is to run xpoz from
source.

### Requirements

1. A Photos.app >= 6.0 library (earlier versions should work with minor changes)
2. Rust
3. NodeJS

### Configuration

If you are running on a Mac and you haven't changed the default location of the
Photos library, then you shouldn't need any configuration. Otherwise, you have
to tell xpoz where your Apple Photos library is located. You can provide
configuration options in several different ways, but for development it's
easiest to create an `.env` file:

    XPOZ_PHOTOS__LIBRARY="/Users/you/Pictures/Photos Library.photoslibrary"
    XPOZ_PHOTOS__DATABASE="/Users/you/Pictures/Photos Library.photoslibrary/database/Photos.sqlite"

I'd also throw a `RUST_LOG=debug` in there for good measure. And please use full
paths - support for tilde substitutions is incomplete.

Finally, create an empty dir from which xpoz will serve static files (the
client):

    $ mkdir public

There are some other configuration options. The defaults are defined in
https://github.com/muxcmux/xpoz/blob/master/src/settings.rs and should be self-
explanatory.

### Server

You need a recent Rust toolchain. Installation instructions are available on the
[Rust lang website](2). Next run `cargo run`. Although subsequent runt are
instant, it does take e bit the first time. If everything goes smoothly you
should end up with a web server running a graphql api on port 1234 and serving
static files from `./static` by default.

Once done, create an access token:

```
$ sqlite3 xpoz.sqlite 'INSERT INTO access VALUES("admin", 0, 1, NULL, "password", NULL);'
```

### Client

xpoz client is a Svelte app. I'm using [Snowpack](3) for development, which
means you will need a modern browser with native modules support, etc.

Navigate to `frontend` and install deps with `yarn`. When done run `yarn dev`
which will compile files into `../public` and fire up a file watcher. With both
the server and the client running, bring up a browser window and go to
`http://localhost:1234/auth?password`.

You should now be able to browse you Apple Photos.app albums from a web browser.

## Why tho?

Because I can and I need something to hack on during lockdown. Plus Photos.app
sharing with people not on the Apple ecosystem (all of my friends/family) sucks
big time. I also had fun building a viewer from scratch, trying out Svelte, Rust
and GraphQL.

## Alpha software

All this is very early version thing and will undergo many chages, depending on
how much free time I have and how much effort I put in it. Contributions are
welcome! Use at your own risk.

## Roadmap

Coming soon or already in the works:

  * ssl support
  * "Admin ui" which allows creating and sharing access tokens easily
  * Pre-built binaries for x86 macs / linux / aarch64

Longer term stuff:

  * More advanced photo viewing experience, pinch actions, desktop zoom, likes,
  comments, etc.
  * More features based on the metadata already available by Apple Photos

[1]: https://www.tonkata.com/posts/apple-photos/
[2]: https://www.rust-lang.org/tools/install
[3]: https://www.snowpack.dev/
