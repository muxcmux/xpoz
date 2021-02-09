# xpoz

An API and a web gallery/viewer for your Apple's Photos.app albums.

I wrote a [blog post](https://www.tonkata.com/posts/apple-photos/)
explaining how some of that stuff works a while back.

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
to tell xpoz where your Apple Photos library is located. You can provide
configuration options in several different ways, but by default xpoz tries to
read configuration from a `settings.yml` in the same directory as the
executable. It is possible to pass a different config file as an argument, eg:

    $: ./xpoz my-config.yml

Here's the default configuration file. Most options should be self explanatory.

```yml
server:
  # toggles a graphql web api explorer at /api (disable for prod instances)
  graphiql: true
  address: 0.0.0.0:1234
  # location of the frontend
  public_dir: ./public
  index_file: index.html

  # if you want to serve photos via https, you need to set this to true + specify the paths to your cert and key
  ssl: false
  key: /path/to/your/key.pem
  cert: /path/to/your/cert.pem

# the following are the defaults and should work in most cases
# but if you have different locations for the library and database
# file, you need to set the appropriate paths to those 
photos:
  library: "~/Pictures/Photos Library.photoslibrary"
  # don't worry this is opened in read only mode
  database: "~/Pictures/Photos Library.photoslibrary/database/Photos.sqlite"
  originals: originals
  renders: resources/renders
  resized: resources/derivatives
  thumbs: resources/derivatives/masters

# this database is used internally and is created automatically
app:
  database: xpoz.sqlite

media:
  # Flip this to true to make transcoded copies of your videos which are
  # suitable for displaying in a browser (AAC/h.264 mp4)
  # (original videos are left intact)
  convert_videos: false
  # you need to have ffmpeg installed with appropriate codecs if you enable
  # video transcoding
  ffmpeg_executable: ffmpeg
  workers: 4
  videos_path: ./videos
```

## Run

    $ ./xpoz

This should have set up and migrated the xpoz database (xpoz.sqlite by default).
Next, you need to set up an admin user:

    $ sqlite3 xpoz.sqlite 'INSERT INTO tokens(name, token, admin, session_bound) VALUES("admin", "password", 1, 0);'

You should now be able to browse you Apple Photos.app albums from a web browser
(http://localhost:1234/auth?password by default).

To create sharable links, visit http://localhost:1234/#/access while
authenticated as an admin.

### Why tho?

Because I can and I need something to hack on during lockdown. Plus Photos.app
sharing with people not on the Apple ecosystem (all of my friends/family) sucks
big time. I also had fun building a viewer from scratch, trying out Svelte, Rust
and GraphQL.

### Alpha software

All this is very early version thing and will undergo many chages, depending on
how much free time I have and how much effort I put in it. Contributions are
welcome! Use at your own risk.
