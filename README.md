# xpoz

An API and a web gallery/viewer for your Apple's Photos.app albums.

I wrote a [blog post](https://www.tonkata.com/posts/apple-photos/)
explaining how some of that stuff works a while back.

It looks like this:

![xpoz demo](xpoz-demo.gif?raw=true)

## Requirements

  1. A Photos.app >= 6.0 library
  2. Mac or a Linux machine
  3. `ffmpeg` (optional)

## Installation

Download a pre-built binary and web assets from the
[releases page](https://github.com/muxcmux/xpoz/releases) for your platform.
Currently there are binaries for MacOS and Linux.

## Configuration

If you are running on a Mac and you haven't changed the default location of the
Photos library, then you shouldn't need any configuration. Otherwise, you have
to tell xpoz where your Apple Photos library is located.

You can provide configuration options in several different ways, but by default
xpoz tries to read configuration from a `settings.yml` in the same directory as
the executable. It is possible to pass a different config file as an argument:

    $ ./xpoz my-config.yml

Please have a look at the [default configuration file](src/default_config.yml)
as a reference for all available options. Any settings in your custom config
file will overwrite these defaults.

## Run

    $ ./xpoz

This should have set up and migrated the xpoz database (xpoz.sqlite by default).
Next, you need to set up an admin user:

    $ sqlite3 xpoz.sqlite 'INSERT INTO tokens(name, token, admin, session_bound) VALUES("admin", "password", 1, 0);'

You should now be able to browse you Apple Photos.app albums from a web browser
(http://localhost:1234/auth?password by default).

To create sharable links, visit http://localhost:1234/#/access while
authenticated as an admin.

If you have `graphiql: true` in your config (the default) you can also visit the
graphql playground and inspect the schema or just fire custom queries. This is
availble at http://localhost:1234/api by default.

### Why tho?

Because I can and I need something to hack on during lockdown. Plus Photos.app
sharing with people not on the Apple ecosystem (all of my friends/family) sucks
big time. I also had fun building a viewer from scratch, trying out Svelte, Rust
and GraphQL.

### Alpha software

All this is very early version thing and will undergo many chages, depending on
how much free time I have and how much effort I put in it. Contributions are
welcome! Use at your own risk.
