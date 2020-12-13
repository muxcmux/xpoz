# xpoz
The result of a mashup of technologies I wanted to learn/play with.

This app is two things really:

1. A backend which serves my Apple Photos.app images as an API.
2. A frontend image viewer / gallery.

The idea, apart from learning all these technologies, is to be able to fire up
a quick server for my images, so they can be shared with friends and family.

I already manage all my photos in Photos.app, but sharing there is kinda weird
and it's a pain if you want to share with anyone who doesn't have an Apple ID.
I also have all the images organised into albums straight into Photos.app, and
have basically zero requirements on authorisation (who can see what) - it's
pretty much sharing everything with anyone I give access to the server.

I do have plans to add more granular sharing functionalities in the future, like
custom links which only allow access to certain albums, etc.

## The backend

This encompasses two of my learning goals: Rust lang and GraphQL. The backend is
a server in Rust, built with Actix. It's really a bunch of creates working
together: sqlx, async-graphql and a few others.

It takes a config yaml with some options, like the destination to the Photos.app
library and database file. Then it exposes two endpoints - a GraphQL endpoint
which is used to get access to albums and images(assets) metadata, and another
endpoint which serves a few different versions of my images.

I wrote a [blog post](1) explaining how some of that stuff works a while back.

This works great, because I have a mini server at home to which my entire
`photoslibrary` syncs automatically. xpoz is hosted on that server and the
config points to the backup. It is behind a https proxy with basic auth.

Currently, the backend works with the db schema of Photos 6.0, which is the one
that ships with Big Sur. I did have to change a couple of table names here and
there to make it work with this Photos.app version, so it should be trivial to
add support for older versions.

## The frontend

Originally I planned to have a go at Elm. I built the backend first and loved
Rust's type system, so plain old JS was out of the question.

I had a play with Elm, but found it to require too much code to get simple
things off the ground. The compiler is absolutely awesome, but at this point
it felt there were too many new things to learn and I just wanted to get this
working soon. I had a look around and decided to try Svelte. It has typescript
support and takes a novelty approach to reactivity.

Anyways, the frontend is quite simple, although I did build my own photo viewer
from scratch (with touch support help from hammerjs). It has basic features at
the moment, which are plenty enough for my needs, but I am actively working on
this project and constantly shipping new features, like phisycs-like pan on
mobile devices, etc. - fun stuff!

### Cool story bro, how do I run this?

Git clone this repo, obviously. You need the standard Rust toolchain and then
build with `cargo build --release`. The resulting binary accepts a config file
as the first an only argument. You can have a look at the defaults in `src/
settings.rs` and use those as keys in a yml to provide custom config.

Frontend is a snowpack thing, so it's basically `cd` into `frontend` and `yarn
start` to run it in development. I'm currently working on building an optimised
prod version with webpack.

#### Alpha software

All this is very early version thing and will undergo many chages, depending on
how much free time I have and how much effort I put in it, so use at your own
risk.

[1]: https://www.tonkata.com/posts/apple-photos/
