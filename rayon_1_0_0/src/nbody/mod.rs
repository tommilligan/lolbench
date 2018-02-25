
use rand::{SeedableRng, XorShiftRng};
use time;

mod bench;
pub use self::bench::*;
mod nbody;
mod visualize;
use self::nbody::NBodyBenchmark;

#[derive(Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum ExecutionMode {
    Par,
    ParReduce,
    Seq,
}

#[derive(Deserialize)]
pub struct Args {
    cmd_bench: bool,
    cmd_visualize: bool,
    flag_mode: Option<ExecutionMode>,
    flag_bodies: usize,
    flag_ticks: usize,
}

fn run_benchmarks(mode: Option<ExecutionMode>, bodies: usize, ticks: usize) {
    let run_par = mode.map(|m| m == ExecutionMode::Par).unwrap_or(true);
    let run_par_reduce = mode.map(|m| m == ExecutionMode::ParReduce).unwrap_or(true);
    let run_seq = mode.map(|m| m == ExecutionMode::Seq).unwrap_or(true);

    let par_time = if run_par {
        let mut rng = XorShiftRng::from_seed([0, 1, 2, 3]);
        let mut benchmark = NBodyBenchmark::new(bodies, &mut rng);
        let par_start = time::precise_time_ns();

        for _ in 0..ticks {
            benchmark.tick_par();
        }

        let par_time = time::precise_time_ns() - par_start;
        println!("Parallel time    : {} ns", par_time);

        Some(par_time)
    } else {
        None
    };

    let par_reduce_time = if run_par_reduce {
        let mut rng = XorShiftRng::from_seed([0, 1, 2, 3]);
        let mut benchmark = NBodyBenchmark::new(bodies, &mut rng);
        let par_start = time::precise_time_ns();

        for _ in 0..ticks {
            benchmark.tick_par_reduce();
        }

        let par_time = time::precise_time_ns() - par_start;
        println!("ParReduce time   : {} ns", par_time);

        Some(par_time)
    } else {
        None
    };

    let seq_time = if run_seq {
        let mut rng = XorShiftRng::from_seed([0, 1, 2, 3]);
        let mut benchmark = NBodyBenchmark::new(bodies, &mut rng);
        let seq_start = time::precise_time_ns();

        for _ in 0..ticks {
            benchmark.tick_seq();
        }

        let seq_time = time::precise_time_ns() - seq_start;
        println!("Sequential time  : {} ns", seq_time);

        Some(seq_time)
    } else {
        None
    };

    if let (Some(pt), Some(st)) = (par_time, seq_time) {
        println!("Parallel speedup : {}", (st as f32) / (pt as f32));
    }

    if let (Some(pt), Some(st)) = (par_reduce_time, seq_time) {
        println!("ParReduce speedup: {}", (st as f32) / (pt as f32));
    }
}

