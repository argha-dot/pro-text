# Pro(tected) Text

This is a clone of [ProtectedText](https://www.protectedtext.com/) written in rust and [Leptos](https://leptos.dev/).

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
  - [ ] Create the User Schema
    > The Working schema for now is
    >
    > ```
    >   username: String,
    >   username_hash: String,
    > ```
  - [ ] Create the user table, API routes etc.
  - [ ] The User and Note relationship etc.
  - [ ] Extend the router with the user stuff
    > The URL will now be something like `https://{ROOT_URL}/{USERNAME}/{NOTE_ID}`
  - [ ] Option to create a new user if the URL isn't already taken
  - [ ] Set Password and Option to make it open i.e. set no password
- [ ] The whole password authentication thing
  - [ ] Encrypt notes and the username with the password.
    > this is stored in the database.
  - [ ] When the user tries to access the note, fetch the username hash, and ask for the password.
  - [ ] Decrypt the username hash with the password taken from the user, and compare it to the username which is known from the url of the site.
  - [ ] If the decrypted username and the actual username match, fetch the list of notes and everything and decrypt it using the password.
    > For now I'm storing the password in a global app state, so that whenerer the page is refreshed or the tab is closed, the user has to login again, but the password is NOT stored in a cookie or local storage. Storing the password this way should not be a security risk, but I'll need to verify it somehow.

## FIX ME
 - [ ] The Flickering of the sidebar.

### Extra Stuff

- [ ] Make the note downloadable.
- [ ] Make importing notes a thing.
- [ ] Make it save stuff as you type.
- [ ] Replace the giant textbox with something like what [notion](https://notion.so) does.

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
