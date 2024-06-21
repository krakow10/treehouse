fn main() {
	for n in 1..=1000000{
		let precision=(std::f32::consts::PI*(n as f32).sqrt()/std::f32::consts::LN_2) as u32+64;
		let pi=rug::Float::with_val(precision,rug::float::Constant::Pi);
		let big_n=rug::Float::with_val(precision,n);
		let val=(pi*big_n.sqrt()).exp();
		let fract=val.fract().to_f64();
		let dist_log=fract.min(1.0-fract).log10();
		if dist_log < -5.0{
			println!("notable: {n}, distance (log10): {dist_log:.2}");
		}
	}
}
