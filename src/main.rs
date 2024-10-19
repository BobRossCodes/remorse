use std::{
    fs::remove_file, io::{stdin, stdout, Write}, path::Path
};

use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
use sound::write_audio_file;
use translation::to_morse;
use words::learning_words;

mod sound;
mod translation;
mod words;

#[derive(Parser, Debug)]
#[clap(name = "Remorse", version("v. 6.9.0"))]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(short, long, global = true)]
    output: Option<String>,

    #[arg(long, action, global = true)]
    open: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    Text {
        text: String,
    },
    Learn {
        #[arg(long, short)]
        letters: Option<String>,

        #[arg(long, short, action)]
        reveal: bool,

        #[arg(long, short, action, requires="reveal")]
        delete: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Text { text } => {
            let as_morse = to_morse(&text);
            println!("{}", as_morse);

            let file_name: String = write_audio_file(&as_morse, args.output)?;

            if args.open {
                opener::open(Path::new(&file_name))?;
            }
        }
        Command::Learn {
            letters: difficult_letters,
            reveal,
            delete
        } => {
            let matched_words = learning_words(difficult_letters)?;

            let word = matched_words
                .choose(&mut rand::thread_rng())
                .expect("empty word list");

            let file_name = write_audio_file(&to_morse(word), args.output)?;

            if args.open {
                opener::open(Path::new(&file_name))?;
            }

            if reveal {
                // wait for user input
                print!("press enter to reveal the word: ");
                stdout().flush()?;
                stdin().read_line(&mut String::new())?;

                println!("original:     {word}");
                println!("morse form:   {}", to_morse(word));
                
                if delete {
                    remove_file(Path::new(&file_name))?;
                    println!("removed file {}", file_name);
                }
            }
        }
    }

    Ok(())
}
