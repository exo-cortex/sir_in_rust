
extern crate derive_more;
use derive_more::{Add, AddAssign, Mul, Div};

#[derive(Copy, Clone, Add, AddAssign, Mul, Div)]
pub struct ModelState {
	pub s: f64, // susceptible
	pub i: f64, // infected
	pub r: f64, // removed i.e. recovered or dead
}

pub struct Parameters {
	pub beta: f64,
	pub gamma: f64,
	pub mu: f64,
}

// #[allow(dead_code)]
// pub fn update_euler(input: &mut ModelState, p: &Parameters, dt: f64) -> () {
// 	let ds = p.mu - p.beta * input.s * input.i - p.mu * input.s;
// 	let di = p.beta * input.s * input.i - p.gamma * input.i - p.mu * input.i;
// 	let dr = p.gamma * input.i - p.mu * input.r;
// 	input.s += dt * ds;
// 	input.i += dt * di;
// 	input.r += dt * dr;
// }

#[allow(dead_code)]
pub fn update_euler(input: &mut ModelState, p: &Parameters, dt: f64) -> () {
	*input += f(&input.clone(), p) * dt;
}

fn f(input: &ModelState, p: &Parameters) -> ModelState {
	return ModelState {
		s: p.mu - p.beta * input.s * input.i - p.mu * input.s, 
		i: p.beta * input.s * input.i - p.gamma * input.i - p.mu * input.i,
		r: p.gamma * input.i - p.mu * input.r
	}
}

#[allow(dead_code)]
pub fn update_rk4(input: &mut ModelState, p: &Parameters, dt: f64) -> () {
	// k1
	let k1 = f(input, p);
	let k2 = f(&(input.clone() + k1 * 0.5 * dt), p);
	let k3 = f(&(input.clone() + k2 * 0.5 * dt), p);
	let k4 = f(&(input.clone() + k3 * dt), p);
	
	*input += (k1 + k2 * 2.0 + k3 * 2.0 + k4) * dt / 6.0;
}

#[allow(dead_code)]
pub fn update_rk4_alt(input: &mut ModelState, p: &Parameters, dt: f64) -> () { 
	// see fn update_rk4
	// here we try to optimize the function
	let mut k1_and_sum = f(input, p) * dt; // we use k1 also to add up the integration step direction
	let mut k234 = f(&(input.clone() + k1_and_sum * 0.5), p) * dt;
	k1_and_sum += k234 * 2.0;

	k234 = f(&(input.clone() + k234 * 0.5), p) * dt;
	k1_and_sum += k234 * 2.0;

	k234 = f(&(input.clone() + k234), p) * dt;
	k1_and_sum += k234;
	
	*input += k1_and_sum / 6.0;
}