# Remorse
A simple Rust CLI that generates a morse code audio file from command line input

# Usage
## Using with cargo
`cargo run -- "Hello world"`
### Default output file name
By default remorse names the files in a format `YYYY-MM-DD HH.MM.wav`
### Specifying output file
`cargo run -- "Hello world" -o "hello.wav"` (-o or --output)
