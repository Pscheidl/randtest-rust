use ::RndTest;
use ::RndTestResult;
use rgsl::randist::gaussian::ugaussian_Pinv;

pub struct DifferencesTestResult{
	pub differences: u64,
	pub criteria: f64
}

impl RndTestResult for DifferencesTestResult{
	fn is_random(&self, alpha: f64) -> bool{
		self.criteria < ugaussian_Pinv( 1f64 - (alpha / 2f64))
	}
}

pub struct DifferencesTest {
	pub rnd_values: Vec<f64>
}

impl RndTest<DifferencesTestResult> for DifferencesTest{
	fn test(&self) -> DifferencesTestResult{
		let differences: u64 = count_positive_diff(&self.rnd_values);
		let criteria: f64 = ((differences as f64 - ((self.rnd_values.len() as f64 - 1f64)/ 2f64)) * (12f64 / (self.rnd_values.len() as f64 + 1f64)).sqrt()).abs();

		DifferencesTestResult{
			differences: differences,
			criteria: criteria
		}

	}
}

fn count_positive_diff(rnd_values: &Vec<f64>) -> u64{
	let mut differences: u64 = 0u64;
	for pos in 1..rnd_values.len(){
		if rnd_values[pos as usize] > rnd_values[pos - 1 as usize]{
			differences += 1;
		}
	}
	differences
}

