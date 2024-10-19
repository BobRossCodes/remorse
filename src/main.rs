use std::{
    fs::{read_dir, remove_file},
    io::{stdin, stdout, Write},
    path::Path,
    time::Duration,
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

    #[arg(short, long, global = true, default_value_t = 80)]
    unit: u64,

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

        #[arg(long, short, action, requires = "reveal")]
        delete: bool,
    },
    Clean,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Text { text } => {
            let as_morse = to_morse(&text);
            println!("{}", as_morse);

            let file_name: String =
                write_audio_file(&as_morse, Duration::from_millis(args.unit), args.output)?;

            if args.open {
                opener::open(Path::new(&file_name))?;
            }
        }
        Command::Learn {
            letters: difficult_letters,
            reveal,
            delete,
        } => {
            let matched_words = learning_words(difficult_letters)?;

            let word = matched_words
                .choose(&mut rand::thread_rng())
                .expect("empty word list");

            let file_name = write_audio_file(
                &to_morse(word),
                Duration::from_millis(args.unit),
                args.output,
            )?;

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
        Command::Clean => {
            let read = read_dir(Path::new("."))?;

            for item in read {
                if let Ok(entry) = item {
                    // try to remove the filename if it ends with .wav
                    if entry.file_name().into_string().unwrap().ends_with(".wav") {
                        remove_file(entry.file_name())?;
                        println!("removed {}", entry.file_name().into_string().unwrap());
                    }
                }
            }
        }
    }

    Ok(())
}
