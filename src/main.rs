// use basic_waves::{NoiseWave, SawWave, SineWave, SquareWave, TriangleWave};
use core::time::Duration;
use rodio::{source::Source, source::SineWave, OutputStream, buffer::SamplesBuffer, Sink};
// use rodio::device::default_output_device;
use std::fs::File;
use std::{io, result};
use std::io::{prelude::*, stdin, stdout, BufReader, Write, BufWriter};
//use std::collections::BTreeMap;
static mut END_POINT: f32 = 3.0;

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
            index,
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

//reads a simple score language file
fn read_lines() -> Vec<String> {
    let mut methods: Vec<String> = vec![];
    let reader = BufReader::new(File::open("src/input.txt").expect("Cannot open file"));

    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            methods.push(word.to_string());
        }
    }
    return methods;
}
// fn get_freq(){
//     let mut frames = 1000;
//     let mut phInc = 2.0 * std::f32::consts::PI * 440.0 / 48000.0;
//      for i in 0..frames{
//         let y = self._phase.sin();
//         _phase += phaseInc;
//         self._queue.push(y as i32 as char);
//      }
// }

fn run_oscillator(a: i32, b: i32, end: f32) -> f32 {
    let wave_table_size = 44100;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    // println!("{}", wave_table[0]);
    for n in 0..wave_table_size {
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
    // let mut oscillator = WavetableOscillator::new(44100, wave_table.clone(),end);
    // //part 6
    // let mut oscillator2 = WavetableOscillator::new(44100, wave_table.clone(),end);
    // let mut oscillator_storage: Vec<WavetableOscillator> = vec![];
    oscillator.set_frequency(get_frequency(a));
    // oscillator2.set_frequency(get_frequency(a+6));
    // // oscillator_storage.push(oscillator);
    // // oscillator_storage.push(oscillator2);

    // //uncomment for part 3
    // // if a >= 0 {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // // for i in 0..oscillator_storage.len(){
    //     let i =1;
    //     let _result = stream_handle.play_raw(oscillator.convert_samples());
    //     std::thread::sleep(std::time::Duration::from_millis(b as u64));
    // // }
    //uncomment for part 3
    //}
    let sine_wave = SineWave::new(440);
    let buffer = SamplesBuffer::new(1, 44000, sine_wave);
    let _result = stream_handle.play_raw(buffer);
    return end;
}

fn main() {
    //part 1 (Plays 440Hz note for 3 seconds)
    // let a = [69];
    // let b = [100];
    //part 2 (Plays Nokia Ringtone)
    let a  = [76, 74, 66, 68, 73, 71, 62, 64, 71, 69, 61, 64, 69];
    let b = [150, 150, 300, 300, 150, 150, 300, 300, 150, 150, 300, 300, 600];
    // let mut b: Vec<i32> = vec![];
    // for i in 0..a.len(){
    //     b.push(10000);
    // }
    let mut end_point: f32 = 0.0;
    //part 3
    // let mut a: Vec<i32> = vec![];
    // let mut b: Vec<f64> = vec![];
    // for i in 0..13{
    //     b.push(300.0);
    // }
    // let words = read_lines();
    // let mut iterator = 0;
    // while iterator < words.len(){
    //     if words[iterator].eq("NoteOn"){
    //         // println!("On");
    //         b.push(words[iterator+1].parse::<f64>().unwrap()*1000.0);
    //         a.push(words[iterator+2].parse().unwrap());
    //     }
    //     else if words[iterator].eq("NoteOff"){
    //             // println!("Off");
    //         b.push(words[iterator+1].parse::<f64>().unwrap()*1000.0);
    //         a.push(-1);
    //         }
    //     // println!("{}",words[iterator]);
    //     iterator+=1;
    // } 
    for it in a.iter().zip(b.iter()) {
        let (ai, bi) = it;
        println!("--------------");
        end_point = run_oscillator(*ai, *bi as i32, end_point);
        // end_point = run_oscillator(*ai+12, *bi as i32, end_point);
    }
}
