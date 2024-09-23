use std::{collections::HashMap, io::stdin};

struct Measurement {
    pub max: f32,
    pub min: f32,
    pub total: f32,
    pub count: f32,
}

fn main() {
    let lines = stdin().lines();
    let mut hash_map = HashMap::with_capacity(10_000);

    for line in lines {
        let line = line.unwrap();
        let (name, temp) = line.split_once(';').unwrap();
        let measurement = hash_map.entry(name.to_string()).or_insert(Measurement {
            max: f32::MAX,
            min: f32::MIN,
            total: 0f32,
            count: 0f32,
        });

        let temp = temp.parse::<f32>().unwrap();

        measurement.max = temp.max(measurement.max);
        measurement.min = temp.min(measurement.min);
        measurement.total += temp;
        measurement.count += 1f32;
    }

    for (station, measurement) in hash_map.iter() {
        println!(
            "{};{};{};{}",
            station,
            measurement.min,
            measurement.total / measurement.count,
            measurement.max
        );
    }
}
