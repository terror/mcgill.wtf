### mcgill.wtf

<div>
  <img width='100px' src='https://super-static-assets.s3.amazonaws.com/6296dc83-05b5-4ba9-bd53-80e15dc04936/images/2da96950-23a6-41d9-bf58-3b65a4ee3737.png'>
</div>

**mcgill.wtf** is a fast full-text search of [mcgill](https://mcgill.ca)'s
entire course catalog with a server implemented in [rust](https://www.rust-lang.org/)
and an intuitive front-end built using [react](https://reactjs.org/).

### Development

#### Downloading course data

In order the query the server for courses, you need to specify a data-source,
which can be downloaded via the `download` subcommand provided by the server
binary:

```bash
$ RUST_LOG=info cargo run -- download --starting-page 0
```

#### Spawning the front and back-end components

The server listens for requests on port `7500` and can be spawned by running the
following:

```bash
$ RUST_LOG=info cargo run -- serve --local --datasource data.json
```

The front-end is now able to issue requests to the server, launch via the
following commands:

```bash
$ npm run install
$ npm run dev
```

### Credits

This project was heavily inspired by Eric Zhang's implementation of
[classes.wtf](https://classes.wtf), and was also built for some of the same
reasons.
