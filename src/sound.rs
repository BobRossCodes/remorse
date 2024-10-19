use std::{f32::consts::PI, i16, time::Duration};

use chrono::Local;
use hound::WavSpec;

const SAMPLE_RATE: u32 = 44100;
const FREQUENCY: f32 = 600.0;

fn default_file_name(unit: &Duration) -> String {
    let date_time = Local::now();
    format!("{} ({:?}).wav", date_time.format("%Y-%m-%d %H.%m.%S"), unit)
}

fn duration_to_samples(d: Duration) -> u32 {
    (d.as_secs_f32() * SAMPLE_RATE as f32) as u32
}

pub fn write_audio_file(
    s: &str,
    unit: Duration,
    output_file: Option<String>,
) -> anyhow::Result<String> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let file_name = output_file.unwrap_or_else(|| default_file_name(&unit));
    let mut writer = hound::WavWriter::create(&file_name, spec)?;

    println!("writing to {file_name}...");
    println!("sample rate:  {SAMPLE_RATE} Hz");
    println!("unit length:  {} ms", unit.as_millis());

    for c in s.chars() {
        match c {
            '.' => {
                for t in 0..duration_to_samples(unit) {
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
                for t in 0..duration_to_samples(unit * 3) {
                    let sample = (t as f32 / SAMPLE_RATE as f32 * FREQUENCY * 2.0 * PI).sin();
                    let amplitude = i16::MAX as f32;

                    writer.write_sample((sample * amplitude) as i16).unwrap();
                }
            }
            ' ' => {
                for _ in 0..duration_to_samples(unit * 3) {
                    writer.write_sample(0)?;
                }
            }
            '/' => {
                // the word space is 7 units, but in the implementation it sleeps three times: " / " -> 3 + 1 + 3 = 7
                for _ in 0..duration_to_samples(unit) {
                    writer.write_sample(0)?;
                }
            }
            _ => {}
        }

        for _ in 0..duration_to_samples(unit) {
            writer.write_sample(0)?;
        }
    }

    writer.finalize()?;
    println!("wrote file successfully.");

    Ok(file_name)
}
