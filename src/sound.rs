use std::{f32::consts::PI, i16};

use chrono::Local;
use hound::WavSpec;

const SAMPLE_RATE: u32 = 44100;
const FREQUENCY: f32 = 600.0;

const SHORT_MILLIS: f32 = 80.0;
const LONG_MILLIS: f32 = 200.0;

fn default_file_name() -> String {
    let date_time = Local::now();
    format!("{}.wav", date_time.format("%Y-%m-%d %H.%m"))
}

pub fn write_audio_file(s: &str, output_file: Option<String>) -> anyhow::Result<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_file.unwrap_or_else(default_file_name), spec)?;

    for c in s.chars() {
        match c {
            '.' => {
                for t in 0..(SAMPLE_RATE as f32 * SHORT_MILLIS / 1000.0) as usize {
                    let sample = (t as f32 / SAMPLE_RATE as f32 * FREQUENCY * 2.0 * PI).sin();
                    let amplitude = i16::MAX as f32;

                    writer.write_sample((sample * amplitude) as i16)?;
                }
            }
            '-' => {
                for t in 0..(SAMPLE_RATE as f32 * LONG_MILLIS / 1000.0) as usize {
                    let sample = (t as f32 / SAMPLE_RATE as f32 * FREQUENCY * 2.0 * PI).sin();
                    let amplitude = i16::MAX as f32;

                    writer.write_sample((sample * amplitude) as i16).unwrap();
                }
            }
            ' ' => {
                for _ in 0..(SAMPLE_RATE as f32 * LONG_MILLIS * 2.0 / 1000.0) as usize {
                    writer.write_sample(0)?;
                }
            }
            '/' => {
                for _ in 0..(SAMPLE_RATE as f32 * LONG_MILLIS * 4.0 / 1000.0) as usize {
                    writer.write_sample(0)?;
                }
            }
            _ => {}
        }

        for _ in 0..(SAMPLE_RATE as f32 * SHORT_MILLIS / 1000.0) as usize {
            writer.write_sample(0)?;
        }
    }

    writer.finalize()?;

    Ok(())
}
