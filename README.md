# Pro(tected) Text

This is a clone of [ProtectedText](https://www.protectedtext.com/) written in rust and [Leptos](https://leptos.dev/).

##

## Todo

- [x] Create Basic UI for notes and sidebar.
- [x] Create the Notes Schema.
- [x] Get sqlx up and running.
- [x] Create basic server functions for getting note, getting all note metadatas and creating note.
- [x] Extend the sidebar with an input form to create notes and a list of notes.
- [x] Create the router, right now without any user, and make it navigatable.
  > The URL will now be something like `https://{ROOT_URL}/{NOTE_ID}`
- [X] Make the text editable and save it in the database so that it functions at least like a notepad thingy for now.
- [X] Replace the default fetching thing with [leptos_query](https://leptos-query-demo.vercel.app/).
- [X] Add ability to delete a Note.
- [ ] USER
  - [X] Create the User Schema
    > The Working schema for now is
    >
    > ```
    >   username: String,
    >   password_hash: String,
    > ```
  - [X] Create the user table, API routes etc.
  - [X] The User and Note relationship etc.
  - [X] Extend the router with the user stuff
    > The URL will now be something like `https://{ROOT_URL}/{USERNAME}/note/{NOTE_ID}`
  - [X] Option to create a new user if the URL isn't already taken
  - [ ] Make a functioning auth
  - [ ] Make Error States

## The Actual Stuff
 - [ ] Replace the giant textbox with a dynamic markdown editor thing.
 - [ ] Steal UI from Notion.
 - [ ] Links inside the notes should be navigable.

## FIX ME
 - [X] The Flickering of the sidebar.
 - [X] The Login Form Input label thing
 - [X] The Router and everything

### Extra Stuff
- [ ] Make importing notes a thing.
- [ ] Make it save stuff as you type.
- [ ] Make the note downloadable.
- [ ] PWA

## Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
pro-text
site/
```

Set the following environment variables (updating for your project as needed):

```text
LEPTOS_OUTPUT_NAME="pro-text"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.
