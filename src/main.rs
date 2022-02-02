use std::fs::File;
use std::io::prelude::*;
use std::env;

mod model;
mod analysis;

fn main()-> std::io::Result<()> {
	
	let args: Vec<String> = env::args().collect();
	for arg in &args {
		println!("{}", arg);
	}
	
	let mut files: Vec<File> = vec![]; 
	// vec![File::create(&filename_s)?, File::create(&filename_i)?, File::create(&filename_r)?];

	let file_number = 3;
	let filenames: Vec<String> = vec!["timeseries_s.txt".to_string(), "timeseries_i.txt".to_string(), "timeseries_r.txt".to_string()];
	for i in 0..file_number {
		files.push(File::create(&filenames[i])?);
	}

	// let filename_s = "foo_rk4_s.txt".to_string();
	// let filename_i = "foo_rk4_i.txt".to_string();
	// let filename_r = "foo_rk4_r.txt".to_string();
	
	const EPSILON: f64 = 0.00001; 
	
	let system_parameters : model::Parameters = model::Parameters { beta: 0.005, gamma: 0.002, mu: 0.001};
	
	let dt: f64 = 1.0 / 128.0;
	
	let mut state: model::ModelState = model::ModelState {s: 0.99, i: 0.01, r: 0.0};
	
	let segment_length: usize = 4096 * 16;
	let segments: usize = 1000 / 8;

	// let mut timeseries: Vec<f64> = vec![0.0_f64; segment_length];
	let mut timeseries: Vec<Vec<f64>> = vec![vec![0.0_f64; 3]; segment_length];
	
	let mut current_time: f64 = 0.0;
	let mut start_time: f64;

	let do_simplification: bool = true;

	let line = format!("{0:.8}\t{1:.8}\n", current_time, state.s);
	write!(files[0], "{}", line).expect("no file possible.");
	let line = format!("{0:.8}\t{1:.8}\n", current_time, state.i);
	write!(files[1], "{}", line).expect("no file possible.");
	let line = format!("{0:.8}\t{1:.8}\n", current_time, state.r);
	write!(files[2], "{}", line).expect("no file possible.");


	for _segment in 0..segments {
		start_time = current_time;
		for i in 0..segment_length {
			model::update_rk4(&mut state, &system_parameters, dt);
			timeseries[i] = [state.s, state.i, state.r].to_vec();
			current_time += dt;
		}
		if do_simplification {
			analysis::write_simplified_timeseries(&timeseries, 0, segment_length - 1, start_time, dt, EPSILON, &mut files);
		}
	}
	Ok(())
}