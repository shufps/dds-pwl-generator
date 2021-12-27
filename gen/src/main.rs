use std::f64::consts;
use std::fs::File;
use std::io::Write;

const DDS_FREQ : f64 = 1_000.0;         // Hz
const SAMPLING_RATE : f64 = 44_100.0;   // Hz
const BITS_ADC : u32 = 12;              // bits
const VOLTS_ADC : f64 = 3.3;            // V
const DURATION : f64 = 100.0;           // ms


const TABLE_SIZE : usize = 256;         // must be power of 2

fn main() {
    let mut dds_table = [0_f64; TABLE_SIZE];

    for i in 0..TABLE_SIZE {
        dds_table[i] = (2.0 * consts::PI / TABLE_SIZE as f64 * i as f64).sin();
        // println!("{} {:.6}", i, dds_table[i]);
    }
 
    let steps = (DURATION / 1000.0 * SAMPLING_RATE) as usize;
    let mut akku = 0_u64;
    let akku_inkr = (DDS_FREQ / SAMPLING_RATE * TABLE_SIZE as f64 * (1_u64<<32) as f64).ceil() as u64;
    // println!("{:016x}", akku_inkr);

    let mut w = File::create("dds.pwl").unwrap();

    let mut last = VOLTS_ADC / 2.0;
    for i in 1..steps {
        let time = (i as f64 / SAMPLING_RATE * 1_000_000_000.0) as u64; // nanoseconds
        akku += akku_inkr;
        let index = (akku as usize >> 32) & (TABLE_SIZE - 1);
        let mut data = dds_table[index] * VOLTS_ADC / 2.0 + VOLTS_ADC / 2.0;
        // ADC quantisierung
        let q = VOLTS_ADC / (1<<BITS_ADC) as f64;
        data = (data / q).ceil() * q;
        let _ = writeln!(&mut w, "{}n {:.6}", time-1, last);     // creates nice steps
        let _ = writeln!(&mut w, "{}n {:.6}", time, data); 
        last = data;
    }
    
}

