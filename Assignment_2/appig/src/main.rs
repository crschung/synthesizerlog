// use basic_waves::{NoiseWave, SawWave, SineWave, SquareWave, TriangleWave};
use core::time::Duration;
use rodio::{source::Source, OutputStream, buffer::SamplesBuffer, Sink};
use std::fs::{File, OpenOptions};
use std::{io, result};
use std::io::{prelude::*, stdin, stdout, BufReader, Write, BufWriter};
//use std::collections::BTreeMap;
use std::path::Path;
use rand::seq::SliceRandom;
use rand::thread_rng;
use hound;

struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>, index:f32) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        // println!("{}",sample);
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index;
        let next_index = (truncated_index + 1.0) % self.wave_table.len() as f32;

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index as usize]
            + next_index_weight * self.wave_table[next_index as usize];
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }
    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }
    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }
    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}

fn get_frequency(value: i32) -> f32 {
    let frequency: f32;
    if value > 0 {
        let power = (value as f32 - 69.00) / 12.00;
        frequency = (440.00 * 2.00_f32.powf(power)).into();
        // println!("Frequency of {} is: {:.2}", value,frequency);
    } else {
        frequency = -1.0;
    }
    return frequency;
}

fn run_oscillator(a: i32, b: i32, mut end: f32) {
    let wave_table_size = 1024;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    for n in 0..wave_table_size{
            wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
        //TODO: Noise
        // wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32 ) - std::f32::consts::PI/std::f32::consts::PI);
        //sawtooth
        // wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32 ) - std::f32::consts::PI/std::f32::consts::PI);
        //square
        // wave_table.push(((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32 ) - std::f32::consts::PI/std::f32::consts::PI).signum());
        //triangle
        // wave_table.push(1.0 - 2.0 * ((std::f32::consts::PI * n as f32 / wave_table_size as f32) - std::f32::consts::PI / std::f32::consts::PI).abs());
    }
    let mut file = File::create("output.txt").unwrap();

    // for val in wave_table.clone() {
    //     let line = format!("{}\n", val);
    //     file.write(line.as_bytes()).unwrap();
    // }
    let mut oscillator = WavetableOscillator::new(44100, wave_table,0.0);

        oscillator.set_frequency(get_frequency(a));
        // oscillator.set_frequency(get_frequency(a+12));
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let _result = stream_handle.play_raw(oscillator.convert_samples());
        
        // let spec = hound::WavSpec {
        //     channels: 1,
        //     sample_rate: 44100,
        //     bits_per_sample: 16,
        //     sample_format: hound::SampleFormat::Int,
        // };
        // let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
        // for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
        //     let sample = (t * 440.0 * 2.0 * std::f32::consts::PI).sin();
        //     let amplitude = i16::MAX as f32;
        //     writer.write_sample((sample * amplitude) as i16).unwrap();
        // }
        std::thread::sleep(std::time::Duration::from_millis(b as u64));
}

fn arpeggiatorInput() -> Vec<i32>{
    let mut numbers = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");

    let mut midiNotes: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    let mut line = String::new();
    numbers.clear();
    println!("Select an arpiggiator mode: \n 1. Upwards \n 2. Downwards \n 3. Up/Down \n 4. Random");
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read input.");
    let mode = line.trim().parse::<i32>().expect("invalid input");
    match mode{
        1 => midiNotes = upwards(midiNotes),
        2 => midiNotes = downwrads(midiNotes),
        3 => midiNotes = upDown(midiNotes),
        4 => midiNotes = randomSeq(midiNotes),
        _ => println!("Invalid Number. Defaulting to order that was put in")
    }
    return midiNotes
}

fn upwards(mut midiNotes: Vec<i32>) -> Vec<i32>{
    midiNotes.sort();
    return midiNotes
}

fn downwrads(mut midiNotes: Vec<i32>) -> Vec<i32>{
    midiNotes.sort();
    midiNotes.reverse();
    return midiNotes
}

fn upDown(mut midiNotes: Vec<i32>) -> Vec<i32>{
    midiNotes.sort();
    let mut midiNotesDown: Vec<i32> = midiNotes.clone();
    midiNotesDown.pop();
    midiNotesDown.reverse();
    midiNotes.append(&mut midiNotesDown);
    // let midiNotesCon = midiNotesUp + midiNotesDown;
    return midiNotes
}

fn randomSeq(mut midiNotes: Vec<i32>) -> Vec<i32>{
    let mut midiNotes2: Vec<i32> = vec![];
    let mut max = midiNotes[0];
    let mut rng = thread_rng();
    midiNotes.shuffle(&mut rng);
    return midiNotes
}

fn main(){
    let mut line = String::new();
    println!("Input MIDI Notes followed by Enter when finished");
    let mut midiNotes = arpeggiatorInput();
    let mut triggerStop = false;
    let mut i = 0;
    println!("Input a tempo modifier (i.e. 0.5 for 2x faster or 2 for 2x slower");
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read input.");
    let tempo = line.trim().parse::<f32>().expect("invalid input");
    let mut tempoModified = 1000.0 * tempo;
    let mut end_point = 0.0;

        for it in 0..midiNotes.len() {
                println!("{}",midiNotes[it]);
                run_oscillator(midiNotes[it], tempoModified as i32, end_point);

    }
}
