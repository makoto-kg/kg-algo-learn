use rand::Rng;
use std::time::Instant;
use floating_duration::TimeAsFloat;

fn main() {
    println!("hello");

    let loop_max = 10000000;
    let test_max = 100;

    let mut x: f32;
    let mut y: f32;

    let mut rng = rand::thread_rng();

    let mut split_time: f32 = 0.0;
    for _test_count in 1..=test_max {
        println!("times:{:?}", _test_count);

        let start_time = Instant::now();

        let mut count = 0;

        for _loop_count in 1..=loop_max {
            x = rng.gen_range(0.0..1.0);
            y = rng.gen_range(0.0..1.0);

            if x * x + y * y <= 1.0 {
                count = count + 1
            }
        }
        println!("pi:{}", 4.0 * count as f32 / loop_max as f32);
        let end_time = start_time.elapsed().as_fractional_secs();

        split_time = split_time + end_time as f32;
    }

    println!("AVE:{}", split_time / test_max as f32);
}