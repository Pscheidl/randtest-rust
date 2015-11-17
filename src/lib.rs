extern crate rgsl;
mod tests;
use tests::ExtremesTest;
use tests::ExtremesTestResult;



pub trait RndTestResult {
	fn is_random(&self, alpha: f64) -> bool;
}

pub trait RndTest<T> where T: RndTestResult{
	fn test(&self) -> T;
}

#[test]
fn text_extreme_points() {
	let vector: Vec<f64> = vec![18f64,78f64,50f64,32f64,23f64,71f64,49f64,45f64,94f64,70f64,81f64,2f64,43f64,80f64,97f64,5f64,51f64,85f64,31f64,49f64,94f64,37f64,74f64,3f64,29f64,91f64,24f64,80f64,8f64,92f64,34f64,76f64,29f64,73f64,44f64,22f64,47f64,95f64,43f64,26f64,44f64,10f64,20f64,18f64,94f64,8f64,2f64,32f64,57f64,95f64,57f64,68f64,97f64,90f64,68f64,83f64,26f64,56f64,70f64,55f64,46f64,19f64,24f64,16f64,22f64,43f64,71f64,17f64,84f64,97f64,93f64,89f64,11f64,65f64,98f64,37f64,11f64,36f64,15f64,1f64,57f64,73f64,6f64,35f64,38f64,43f64,98f64,73f64,61f64,20f64,37f64,25f64,39f64,24f64,24f64,18f64,31f64,6f64,98f64,76f64];
	let test: ExtremesTest = ExtremesTest{rnd_values: vector};
	let result: ExtremesTestResult = test.test();
	assert_eq!(63u64, result.extreme_pts);
	assert_eq!(0.5584829866163423f64, result.criteria);
	assert!(result.is_random(0.05));

}

