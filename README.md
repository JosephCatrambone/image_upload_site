
## Design:

#### CUJs: 
- Sign up
- Log in
- Upload Image
- Add description
- Vote on description
- Add tag
- Vote on tag
- Export global dataset

## Developer Notes, for Developers!:

### Setting Up:
`cargo install perseus-cli --version 0.4.0-beta.18`

`cargo install wasm-pack` is done automatically by the above.

### Building for Debug:

`perseus serve -w` for reactive bits.

`perseus export -sw` for just watching static files without reactivity.

### Building for Release:

`perseus release`