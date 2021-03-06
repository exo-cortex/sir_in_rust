// todo:
// -find extrema
// -count extrema
// -write simplification function for parameteric plots where samples are not equidistant in time 

extern crate derive_more;
use derive_more::{Add, Mul}; // AddAssign, MulAssign, Div

use std::fs::File;
use std::io::prelude::*;

#[derive(Add, Mul)]
pub struct Point2 {
	x: f64,
	y: f64,
}

fn perpendicular_distance_square(a: &Point2, b: &Point2, point: &Point2) -> f64 {
	// returns the perpendicular distance of a line through A, B to point
	return ((b.x - a.x) * (a.y - point.y) - (a.x - point.x) * (b.y - a.y)).powi(2) 
	/ ((b.x - a.x).powi(2) + (b.y - a.y).powi(2));
	// normal formula would be: ((b.x - a.x) * (a.y - point.y) - (a.x - point.x) * (b.y - a.y)).abs() 
	// / ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
}

// do rdp (ramer-douglas-peucker) curve simplification
// assumption: a timeseries has x values in an ascending, equidistant order
pub fn write_simplified_timeseries(
	point_line: &Vec<Vec<f64>>,
	start_index: usize,
	end_index: usize,
	start_time: f64,
	dt: f64,
	epsilon: f64,
	outfiles: &mut Vec<File>) -> () {
	for i in 0..point_line[0].len() {
		simplify_timeseries(point_line, i, start_index, end_index, start_time, dt, epsilon.powi(2), &mut outfiles[i]);
		let line = format!("{0:.8}\t{1:.8}\n", start_time + dt * (end_index as f64), point_line[end_index][i]);
		write!(&mut outfiles[i], "{}", line).expect("no file possible.");
	}
}

pub fn simplify_timeseries(
	point_line: &Vec<Vec<f64>>, 
	coordinate_index: usize,
	start_index: usize, 
	end_index: usize, 
	start_time: f64, 
	dt: f64, 
	epsilon_square: f64,
	outfile: &mut File) -> () {
	
	let mut max_distance_square: f64 = 0.0;
	let mut index_of_max: usize = start_index + 1;
	let a = Point2 {x: start_time + dt * (start_index as f64), y: point_line[start_index][coordinate_index]};
	let b = Point2 {x: start_time + dt * (end_index as f64), y: point_line[end_index][coordinate_index]};
	let mut p: Point2;
	let mut current_time: f64 = start_time + dt * (start_index as f64);

	for i in (start_index + 1)..(end_index) {
		current_time += dt;
		p = Point2 {x: current_time, y: point_line[i][coordinate_index]};
		let d_square = perpendicular_distance_square(&a, &b, &p);
		if max_distance_square < d_square {
			max_distance_square = d_square;
			index_of_max = i;
		}	
	}

	let line = format!("{0:.8}\t{1:.8}\n", a.x, a.y);
	write!(outfile, "{}", line).expect("no file possible.");

	if max_distance_square > epsilon_square {
		simplify_timeseries(&point_line, coordinate_index, start_index, index_of_max, start_time, dt, epsilon_square, outfile);
		simplify_timeseries(&point_line, coordinate_index, index_of_max, end_index, start_time, dt, epsilon_square, outfile);
		return
	}

	return
}

// pub fn write_simplified_curve(
// 	point_line: &Vec<Vec<f64>>,
// 	start_index: usize,
// 	end_index: usize,
// 	start_time: f64,
// 	dt: f64,
// 	epsilon: f64,
// 	outfiles: &mut Vec<File>) -> () {
// 	for i in 0..point_line[0].len() {
// 		simplify_timeseries(point_line, i, start_index, end_index, start_time, dt, epsilon, &mut outfiles[i]);
// 		let line = format!("{0:.8}\t{1:.8}\n", start_time + dt * (end_index as f64), point_line[end_index][i]);
// 		write!(&mut outfiles[i], "{}", line).expect("no file possible.");
// 	}
// }