fn main() {
	for n in 1..=1000000{
		//use however much precision is needed for the calculation + 64 bits of overhead
		let precision=(std::f32::consts::PI*(n as f32).sqrt()/std::f32::consts::LN_2) as u32+64;
		//initialize arbitrary precision numbers
		let pi=rug::Float::with_val(precision,rug::float::Constant::Pi);
		let big_n=rug::Float::with_val(precision,n);
		//compute the value in question
		let val=(pi*big_n.sqrt()).exp();
		//get the fractional part and convert to 64-bit float
		let fract=val.fract().to_f64();
		//find the distance to the closest integer and do log10
		let dist_log=fract.min(1.0-fract).log10();
		//print notable values of n and their log10 distance from an integer
		if dist_log < -5.0{
			println!("notable: {n}, distance (log10): {dist_log:.2}");
		}
	}
}
