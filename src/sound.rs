use std::{f32::consts::PI, i16};

use chrono::Local;
use hound::WavSpec;

const SAMPLE_RATE: u32 = 44100;
const FREQUENCY: f32 = 600.0;

const DOT_MILLIS: f32 = 80.0;
const DASH_MILLIS: f32 = 200.0;
const LETTER_SPACE_MILLIS: f32 = DOT_MILLIS;

fn default_file_name() -> String {
    let date_time = Local::now();
    format!("{}.wav", date_time.format("%Y-%m-%d %H.%m.%S"))
}

pub fn write_audio_file(
    s: &str,
    output_file: Option<String>,
) -> anyhow::Result<String> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let file_name = output_file.unwrap_or_else(default_file_name);
    let mut writer = hound::WavWriter::create(&file_name, spec)?;

    println!("writing to {file_name}...");
    println!("sample rate:  {SAMPLE_RATE} (Hz)");
    println!("dot length:   {DOT_MILLIS} (ms)");
    println!("dash length:  {DASH_MILLIS} (ms)");
    println!("letter space: {LETTER_SPACE_MILLIS} (ms)");

    for c in s.chars() {
        match c {
            '.' => {
                for t in 0..(SAMPLE_RATE as f32 * DOT_MILLIS / 1000.0) as usize {
                    // t / SAMPLE_RATE = [s] = time in seconds
                    // 
                    // one period for sin is 2 * pi
                    // therefore 1 Hz would equal to t / SAMPLE RATE * 2 * pi
                    // and FREQUENCY [Hz] = t / SAMPLE_RATE * 2 * pi * FREQUENCY


                    let sample = (t as f32 / SAMPLE_RATE as f32 * FREQUENCY * 2.0 * PI).sin();
                    let amplitude = i16::MAX as f32;

                    writer.write_sample((sample * amplitude) as i16)?;
                }
            }
            '-' => {
                for t in 0..(SAMPLE_RATE as f32 * DASH_MILLIS / 1000.0) as usize {
                    let sample = (t as f32 / SAMPLE_RATE as f32 * FREQUENCY * 2.0 * PI).sin();
                    let amplitude = i16::MAX as f32;
                    
                    writer.write_sample((sample * amplitude) as i16).unwrap();
                }
            }
            ' ' => {
                for _ in 0..(SAMPLE_RATE as f32 * DASH_MILLIS * 2.0 / 1000.0) as usize {
                    writer.write_sample(0)?;
                }
            }
            '/' => {
                for _ in 0..(SAMPLE_RATE as f32 * DASH_MILLIS * 4.0 / 1000.0) as usize {
                    writer.write_sample(0)?;
                }
            }
            _ => {}
        }

        for _ in 0..(SAMPLE_RATE as f32 * LETTER_SPACE_MILLIS / 1000.0) as usize {
            writer.write_sample(0)?;
        }
    }

    writer.finalize()?;
    println!("wrote file successfully.");

    Ok(file_name)
}
