# pixelspammer
A simple, multithreaded pixelflut client to spam images in slices

## Usage
### Without installation
Just run it with cargo directly:
```
$ cargo run --release -- -h [IP:port] -i [/path/to/image.png] -s [number of slices] (-x [x-offset]) (-y [y-offset])
```
Or to get detailed help for the options:
```
$ cargo run --release -- --help
```

### With installation
Install it (and make sure that **~/.cargo/bin** is in your **PATH**):
```
$ cargo install --path .
```
And then run it:
```
$ pixelspammer -h [IP:port] -i [/path/to/image.png] -s [number of slices] (-x [x-offset]) (-y [y-offset])
```
Or to get detailed help for the options:
```
$ pixelspammer --help
```