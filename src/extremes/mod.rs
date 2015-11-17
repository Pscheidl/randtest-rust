use ::RndTest;
use ::RndTestResult;
use rgsl::randist::gaussian::ugaussian_Pinv;

pub struct ExtremesTestResult {
	pub extreme_pts: u64,
	pub criteria: f64,
}

impl RndTestResult for ExtremesTestResult{
	fn is_random(&self, alpha: f64) -> bool{
		self.criteria < ugaussian_Pinv( 1f64 - (alpha / 2f64))
	}
}

pub struct ExtremesTest{
	pub rnd_values: Vec<f64>
}

impl RndTest<ExtremesTestResult> for ExtremesTest{
	fn test(&self) -> ExtremesTestResult{
		let num_ext_pts = extreme_pts(&self.rnd_values);
		let criteria: f64 = ((num_ext_pts as f64 - ((2f64 * (self.rnd_values.len() as f64 - 2f64))/ 3f64)) * (90f64 / (16f64 * self.rnd_values.len() as f64 - 29f64)).sqrt()).abs();
		ExtremesTestResult{
			extreme_pts: num_ext_pts,
			criteria: criteria,
		}

	}
}

fn extreme_pts(pts: &Vec<f64>) -> u64{
	let mut found_pts : u64 = 0u64;

	for pos in 1..pts.len() - 1 as usize{
		if pts[pos] > pts[pos - 1] && pts[pos] > pts[pos + 1] {
			found_pts += 1;
		}
		if pts[pos] < pts[pos - 1] && pts[pos] < pts[pos + 1] {
			found_pts += 1;
		}
	}
	found_pts
}

