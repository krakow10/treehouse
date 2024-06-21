fn main() {
	for n in 1..=1000000{
		let precision=(std::f32::consts::PI*(n as f32).sqrt()/std::f32::consts::LN_2) as u32+64;
		let pi=rug::Float::with_val(precision,rug::float::Constant::Pi);
		let big_n=rug::Float::with_val(precision,n);
		let val=(pi*big_n.sqrt()).exp();
		let int=val.clone().round();
		let diff_log=(val-int).abs().log10().to_f32();
		if diff_log< -5.0{
			println!("notable: {n} digits:{diff_log}");
		}
	}
}
