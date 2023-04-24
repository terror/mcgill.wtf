### mcgill.wtf

<div>
  <img width='100px' src='https://user-images.githubusercontent.com/31192478/234092331-40c9c5d0-8a60-42ef-aab2-e2dfdda80772.png'>
</div>

**mcgill.wtf** is a fast full-text search of [McGill](https://mcgill.ca)'s
entire course catalog with a server implemented in [Rust](https://www.rust-lang.org/)
and an intuitive front-end built using [React](https://reactjs.org/).

### Development

#### Downloading course data

In order the query the server for courses, you need to specify a data-source,
which can be downloaded via the `download` subcommand provided by the server
binary:

```bash
$ RUST_LOG=info cargo run -- download --starting-page <page>
```

#### Spawning the front and back-end components

In order to run the server locally you need [Docker](https://www.docker.com/)
and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
installed on your machine. By default the server listens for requests on port
`7500` and can be spawned by running the `serve` script in the `/bin` directory
located in the project root:

```bash
$ RUST_LOG=info ./bin/serve -l -d <datasource>
```

The front-end is now able to issue requests to the server, launch it by invoking
the following commands:

```bash
$ npm run install
$ npm run dev
```

### Credits

This project was heavily inspired by Eric Zhang's implementation of
[classes.wtf](https://classes.wtf), and was also built for some of the same
reasons.
