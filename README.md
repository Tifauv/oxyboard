Oxyboard
========

## Overview

Oxyboard is a _tribune_, a sort of web-based chat room.

It is written in [Rust](https://www.rust-lang.org) as a toy project to learn the language.
The HTTP server part is handled by [Iron](https://github.com/iron/iron "Iron on GitHub").

## Features

- **Self-contained**. The compilation result is a binary that you can start directly. No need for an application server.
- **No configuration needed**. A default configuration is builtin, that allow you to start a board listening on localhost:8080.
- **REST API**. See the API section below.
- **Fast**. On a modest Core i5-4200M powered laptop, it could handle more around 700 new messages per second.
  And it is still limited by the disk I/O because all writes are synchronous for now.
- **No memory leaks**. Uses 100% safe Rust code.

### API

For now, there is only a basic web interface. The development effort is put on core features.

Basically, a chat room needs two types of requests:

- read the messages,
- send a message.

_Details coming..._

### Threading through timestamps

A _tribune_ has a notion of threads, based on message timestamps.
Each message has a clickable timestamp (formatted as hh:mm:ss^sub).

When a user wants to respond to a message, she clicks on the message's timestamp
before typing her message. This adds the timestamp to the edit widget's current location.
She can also type the timestamp by hand, it is simply text at this point.

For example, say Alice wants to respond to Bob:

```
  | ...                                           |
  | Bob 12:00:42 - Hello !                        |
  |-----------------------------------------------|
  | [ 12:00:42 Hi !                    ] [_Send_] |
  |_______________________________________________|
```

Once she sends her message, the timestamp appears in the message:

```
  | ...                                           |
  | Bob   12:00:42 - Hello !                      |
  | Alice 12:01:31 - 12:00:42 Hi !                |
  |-----------------------------------------------|
  | [                                  ] [_Send_] |
  |_______________________________________________|
```

Now, when hovering the mouse over Bob's message timestamp highlights Alice's reponse.
This way, you can easily view all responses to a message.

Likewise, hovering the timestamp reference '12:00:42' in Alice's message highlights Bob's message.
This way, you can view the previous message in a discussion.

Using this simple but powerfull mechanism, it is possible to follow multiple interleaved discussions.

```
  | ...                                           |
  | Bob   12:00:42 - Hello !                      |
  | Alice 12:01:31 - 12:00:42 Hi !                |
  | Eve   12:01:52 - 12:00:42 _o/                 |
  | Bob   12:02:03 - 12:01:31 12:01:52 \o_        |
  |-----------------------------------------------|
```

## I want to get it !

Being writen in Rust, you need the [Rustc compiler](https://www.rust-lang.org/fr/downloads.html) version 1.13 minimum to build the project. Rust distributions now come with the `Cargo` builder.

Then, you should [get the code on GitHub](https://github.com/Tifauv/oxyboard).
A `cargo run` will take care of downloading the dependencies, building all that and starting a board
on 0.0.0.0:8080.

## Documentation

You can use `cargo doc` to locally build the documentation. It will be created in the target/doc directory.
