use core::time::Duration;
use rodio::{OutputStream, source::Source, Sink};
use rodio::source::{SineWave};
use std::io::{stdin,stdout,Write, prelude::*,BufReader};
use std::{fs::File, path::Path};

struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

struct Sawtooth {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}
struct Pulse {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}
struct Triangle {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}
struct Noise {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}
struct Score{
    message: String,
    time: u64,
    midi_note: i32,
    unknown: i32
}

impl Score {
    fn note_status(){

    }
    fn set_frequency(){

    }
    fn new(message: String, time: u64, midi_note: i32) -> Score{
        return Score {
            message: message,
            time: time,
            midi_note: midi_note,
            unknown: 0
        };
    }
    fn set_time(){

    }
}
impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }
    
    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 
                               / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();
        
        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index] 
               + next_index_weight * self.wave_table[next_index];
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

fn get_frequency(value:i32) -> f32{
    let power = (value as f32-69.00)/12.00;
    let frequency:f32 = (440.00*2.00_f32.powf(power)).into();
    println!("Frequency of {} is: {:.2}", value,frequency);
    return frequency;
}

//reads a simple score language file 
fn read_lines() -> Vec<String>{
    let file = File::open("src/input.txt").expect("No file named input.txt was found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l|l.expect("Couldn't parse line"))
        .collect()
}

fn main() {
    println!("Never Gonna Give You Up!");
    //part 1 (Plays 440Hz note for 3 seconds)
    // let a = [69];
    // let b = [3000];
    //part 2 (Plays Never Gonna Give You Up by Rick Astley)
    let a = [67,69,72,69,76,76,74,67,69,72,69,74,74,72];
    let b = [200,200,200,200,400,400,500,200,200,200,200,400,400,500];
    //part 3
    //input txt file
    //parse file into array
    //assign frequency
    //noteon play freq for n ms
    //noteoff play -1 for n ms
    
    let lines = read_lines();
    for line in lines {
        println!("{}",line);
    }


    for it in a.iter().zip(b.iter()){ 
        let (ai,bi) = it;
        let wave_table_size = 64;
        let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

        for n in 0..wave_table_size {
            wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
        }
        let mut oscillator = WavetableOscillator::new(44100, wave_table);
        if *ai >= 0 {
            oscillator.set_frequency(get_frequency(*ai));
        }
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            //let _result = stream_handle.play_raw(oscillator.convert_samples());
            // let val: u32 = (*ai).try_into().unwrap();
            println!("{}",*ai);
            let test = SineWave::new(get_frequency(*ai)as u32).take_duration(Duration::from_millis(*bi)).amplify(0.20);
            sink.append(test);
            println!("{}",sink.volume());
            sink.set_volume(0.5);
            println!("{}",sink.volume());
            sink.sleep_until_end();
            //std::thread::sleep(std::time::Duration::from_millis(*bi));
    }
}