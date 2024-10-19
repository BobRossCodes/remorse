# Remorse
A simple Rust CLI that generates a morse code audio file from command line input. Can also give random morse code audio samples when using the **learn** sub-command.

# Usage with cargo
## Create a wav file from text
`cargo run -- text "Hello world"`
<br>
By default remorse names the files in a format `YYYY-MM-DD HH.MM.SS.wav`
## Specifying output file
`cargo run -- text "Hello world"`

## Random words
`cargo run -- learn`
<br>
This generates an audio file of a random word.

### Specifying difficult letters
`cargo run -- learn -l abcdef` (-l or --letters)
<br>
This tries to select words that best match the letters you wish to learn.

### Revealing the random word
`cargo run -- learn -r` (-r or --reveal)
#### Deleting the output file automatically
`cargo run -- learn -r -d` (-d or --delete)
<br>
This can only be used when using the --reveal flag.

## Cleaning up .wav files
`cargo run -- clean`
<br>
This will remove all files ending with `.wav` in the current working directory.

## Global arguments
### Output file
-o | --output "my-desired-output-file.wav"`
### Open in default program
--open
