use sound::write_audio_file;
use translation::to_morse;
use clap::Parser;

mod translation;
mod sound;

#[derive(Parser)]
struct Cli {
    text: String,

    #[arg(short, long)]
    output: Option<String>
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let as_morse = to_morse(&args.text);
    println!("{}", as_morse);
    
    write_audio_file(&as_morse, args.output)?;

    Ok(())
}
