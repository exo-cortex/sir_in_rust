use std::fs::File;
use std::env;

mod model;
mod analysis;

fn main()-> std::io::Result<()> {
	
	let mut files: Vec<File> = vec![]; 

	let file_number = 3;
	let filenames: Vec<String> = vec!["timeseries_s.txt".to_string(), "timeseries_i.txt".to_string(), "timeseries_r.txt".to_string()];
	for i in 0..file_number {
		files.push(File::create(&filenames[i])?);
	}
	
	const EPSILON: f64 = 0.0005; 
	
	let mut system_parameters : model::Parameters = model::Parameters { beta: 0.4, gamma: 0.3, mu: 0.01};
	println!("basic reproduction number R_0 = beta / gamma = {}", system_parameters.beta/system_parameters.gamma);
	let dt: f64 = 1.0 / 128.0;
	
	let segment_length: usize = 4096 * 8;
	let mut segments: usize = 20;

	let mut state: model::ModelState = model::ModelState {s: 0.99, i: 0.01, r: 0.0};
	let args: Vec<String> = env::args().collect();
	for i in 0..args.len() {
		// println!("{} -> {}", i, &args[i]);
		if &args[i] == "-beta" && i < args.len() - 1 && !&args[i + 1].parse::<f64>().is_err() {
			system_parameters.beta = args[i + 1].parse().unwrap();
			println!("beta can be parsed");
		}
		if &args[i] == "-gamma" && i < args.len() - 1 && !&args[i + 1].parse::<f64>().is_err() {
			system_parameters.gamma = args[i + 1].parse().unwrap();
			println!("gamma can be parsed");
		}
		if &args[i] == "-mu" && i < args.len() - 1 && !&args[i + 1].parse::<f64>().is_err() {
			system_parameters.mu = args[i + 1].parse().unwrap();
			println!("mu can be parsed");
		}

		if &args[i] == "-segments" && i < args.len() - 1 && !&args[i + 1].parse::<usize>().is_err() {
			segments = args[i + 1].parse().unwrap();
		}
	}



	println!("integrating {} steps.", segments * segment_length);

	let mut timeseries: Vec<Vec<f64>> = vec![vec![0.0_f64; 3]; segment_length];
	
	let mut current_time: f64 = 0.0;
	let mut start_time: f64;

	let do_simplification: bool = true;

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