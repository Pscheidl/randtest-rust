extern crate rgsl;
pub mod extremes;
pub mod differences;
pub mod ks;
use extremes::ExtremesTest;
use extremes::ExtremesTestResult;
use differences::DifferencesTest;
use differences::DifferencesTestResult;
use ks::KolmogorovSmirnovTest;
use ks::KolmogorovSmirnovTestResult;



pub trait RndTestResult {
	fn is_random(&self, alpha: f64) -> bool;
}

pub trait RndTest<T> where T: RndTestResult{
	fn test(&self) -> T;
}

#[test]
fn test_extreme_points() {
	let vector: Vec<f64> = vec![18f64,78f64,50f64,32f64,23f64,71f64,49f64,45f64,94f64,70f64,81f64,2f64,43f64,80f64,97f64,5f64,51f64,85f64,31f64,49f64,94f64,37f64,74f64,3f64,29f64,91f64,24f64,80f64,8f64,92f64,34f64,76f64,29f64,73f64,44f64,22f64,47f64,95f64,43f64,26f64,44f64,10f64,20f64,18f64,94f64,8f64,2f64,32f64,57f64,95f64,57f64,68f64,97f64,90f64,68f64,83f64,26f64,56f64,70f64,55f64,46f64,19f64,24f64,16f64,22f64,43f64,71f64,17f64,84f64,97f64,93f64,89f64,11f64,65f64,98f64,37f64,11f64,36f64,15f64,1f64,57f64,73f64,6f64,35f64,38f64,43f64,98f64,73f64,61f64,20f64,37f64,25f64,39f64,24f64,24f64,18f64,31f64,6f64,98f64,76f64];
	let test: ExtremesTest = ExtremesTest{rnd_values: vector};
	let result: ExtremesTestResult = test.test();
	assert_eq!(63u64, result.extreme_pts);
	assert_eq!(0.5584829866163423f64, result.criteria);
	assert!(result.is_random(0.01));
}

#[test]
fn test_differences() {
	let vector: Vec<f64> = vec![18f64,78f64,50f64,32f64,23f64,71f64,49f64,45f64,94f64,70f64,81f64,2f64,43f64,80f64,97f64,5f64,51f64,85f64,31f64,49f64,94f64,37f64,74f64,3f64,29f64,91f64,24f64,80f64,8f64,92f64,34f64,76f64,29f64,73f64,44f64,22f64,47f64,95f64,43f64,26f64,44f64,10f64,20f64,18f64,94f64,8f64,2f64,32f64,57f64,95f64,57f64,68f64,97f64,90f64,68f64,83f64,26f64,56f64,70f64,55f64,46f64,19f64,24f64,16f64,22f64,43f64,71f64,17f64,84f64,97f64,93f64,89f64,11f64,65f64,98f64,37f64,11f64,36f64,15f64,1f64,57f64,73f64,6f64,35f64,38f64,43f64,98f64,73f64,61f64,20f64,37f64,25f64,39f64,24f64,24f64,18f64,31f64,6f64,98f64,76f64];
	let test: DifferencesTest = DifferencesTest{rnd_values: vector};
	let result: DifferencesTestResult = test.test();
	assert_eq!(50u64, result.differences);
	assert_eq!(0.1723454968864278f64, result.criteria);
	assert!(result.is_random(0.01));
}

#[test]
fn kolmogorov_smirnov(){
	let vector: Vec<f64> = vec![79f64,98f64,12f64,46f64,85f64,95f64,32f64,9f64,79f64,88f64,78f64,48f64,58f64,12f64,3f64,78f64,7f64,37f64,2f64,85f64,8f64,33f64,60f64,80f64,66f64,67f64,62f64,6f64,63f64,50f64,7f64,4f64,88f64,31f64,68f64,32f64,47f64,3f64,6f64,10f64,49f64,35f64,87f64,55f64,50f64,17f64,38f64,44f64,61f64,40f64,66f64,57f64,14f64,42f64,34f64,28f64,40f64,17f64,24f64,86f64,64f64,44f64,6f64,40f64,27f64,26f64,15f64,61f64,67f64,10f64,38f64,35f64,84f64,86f64,60f64,42f64,19f64,58f64,89f64,65f64,80f64,27f64,26f64,7f64,84f64,70f64,96f64,93f64,44f64,24f64,44f64,12f64,74f64,99f64,11f64,98f64,69f64,67f64,57f64,9f64,61f64,20f64,57f64,83f64,82f64,67f64,36f64,65f64,51f64,42f64,80f64,67f64,94f64,27f64,30f64,2f64,33f64,76f64,22f64,90f64,42f64,27f64,46f64,70f64,80f64,62f64,75f64,15f64,63f64,99f64,34f64,48f64,17f64,93f64,28f64,32f64,38f64,32f64,54f64,65f64,36f64,22f64,44f64,34f64,45f64,100f64,61f64,97f64,32f64,63f64,80f64,54f64,25f64,54f64,64f64,47f64,29f64,26f64,82f64,18f64,100f64,62f64,74f64,45f64,59f64,36f64,67f64,74f64,62f64,86f64,92f64,60f64,19f64,30f64,73f64,40f64,7f64,6f64,28f64,83f64,88f64,75f64,38f64,61f64,1f64,52f64,23f64,67f64,74f64,13f64,6f64,15f64,65f64,94f64,59f64,25f64,5f64,98f64,52f64,59f64,35f64,71f64,3f64,77f64,22f64,97f64,23f64,78f64,15f64,77f64,39f64,33f64,72f64,26f64,6f64,20f64,93f64,11f64,10f64,29f64,13f64,72f64,83f64,50f64,60f64,29f64,92f64,21f64,22f64,37f64,74f64,3f64,96f64,32f64,54f64,9f64,62f64,34f64,96f64,98f64,30f64,53f64,90f64,29f64,39f64,82f64,60f64,76f64,27f64,75f64,97f64,55f64,42f64,46f64,60f64,67f64,61f64,53f64,90f64,18f64,60f64,9f64,77f64,9f64,66f64,29f64,14f64,72f64,64f64,65f64,6f64,48f64,31f64,51f64,75f64,98f64,85f64,23f64,42f64,51f64,38f64,67f64,34f64,12f64,62f64,87f64,10f64,96f64,2f64,3f64,77f64,9f64,11f64,70f64,48f64,45f64,36f64,77f64,63f64,84f64,91f64,42f64,59f64,11f64,42f64,28f64,17f64,31f64,49f64,65f64,54f64,25f64,10f64,72f64,36f64,63f64,89f64,96f64,89f64,63f64,98f64,39f64,25f64,99f64,86f64,63f64,5f64,48f64,30f64,50f64,19f64,92f64,40f64,34f64,13f64,33f64,5f64,31f64,42f64,84f64,76f64,3f64,39f64,26f64,96f64,73f64,15f64,28f64,70f64,13f64,85f64,56f64,11f64,59f64,1f64,36f64,29f64,66f64,1f64,66f64,31f64,91f64,37f64,38f64,12f64,86f64,15f64,73f64,67f64,9f64,72f64,20f64,91f64,87f64,11f64,32f64,6f64,65f64,69f64,53f64,45f64,79f64,21f64,100f64,86f64,96f64,98f64,93f64,8f64,8f64,30f64,24f64,48f64,73f64,3f64,78f64,54f64,44f64,62f64,49f64,44f64,61f64,65f64,45f64,54f64,7f64,42f64,84f64,81f64,27f64,32f64,75f64,2f64,2f64,30f64,27f64,33f64,34f64,69f64,54f64,49f64,64f64,24f64,4f64,74f64,34f64,55f64,67f64,75f64,23f64,36f64,95f64,75f64,53f64,77f64,49f64,23f64,15f64,82f64,8f64,13f64,90f64,32f64,50f64,34f64,92f64,54f64,45f64,29f64,13f64,54f64,31f64,98f64,23f64,84f64,32f64,48f64,56f64,35f64,31f64,76f64,80f64,27f64,100f64,77f64,48f64,47f64,47f64,69f64,37f64,36f64,96f64,68f64,28f64,13f64,9f64,75f64,17f64,92f64,16f64,57f64,31f64,14f64,55f64,3f64,24f64,40f64,4f64,51f64,58f64,37f64,49f64,6f64,39f64,37f64,4f64,22f64,27f64,46f64,34f64,57f64,83f64,17f64,8f64,94f64,7f64,6f64,78f64,57f64,39f64,2f64,91f64,32f64,76f64,45f64,12f64,57f64,29f64,34f64,29f64,32f64,100f64,90f64,80f64,45f64,39f64,17f64,70f64,96f64,15f64,29f64,32f64,61f64,75f64,12f64,24f64,23f64,30f64,32f64,91f64,66f64,21f64,73f64,63f64,20f64,72f64,94f64,51f64,5f64,2f64,68f64,25f64,31f64,64f64,75f64,58f64,10f64,85f64,96f64,79f64,14f64,75f64,60f64,11f64,69f64,50f64,47f64,79f64,12f64,14f64,27f64,47f64,42f64,33f64,86f64,40f64,12f64,77f64,83f64,100f64,51f64,7f64,12f64,68f64,89f64,61f64,38f64,76f64,14f64,94f64,64f64,63f64,68f64,13f64,82f64,47f64,39f64,85f64,71f64,11f64,71f64,60f64,72f64,27f64,34f64,23f64,100f64,87f64,42f64,64f64,73f64,45f64,95f64,6f64,5f64,69f64,8f64,18f64,83f64,77f64,56f64,16f64,37f64,79f64,82f64,6f64,13f64,6f64,6f64,90f64,35f64,6f64,37f64,69f64,51f64,24f64,74f64,57f64,68f64,23f64,53f64,17f64,21f64,83f64,58f64,67f64,83f64,35f64,62f64,12f64,56f64,79f64,1f64,3f64,68f64,20f64,11f64,73f64,69f64,58f64,11f64,46f64,90f64,37f64,27f64,34f64,45f64,64f64,51f64,81f64,35f64,1f64,24f64,13f64,43f64,16f64,4f64,25f64,6f64,98f64,15f64,52f64,42f64,23f64,63f64,46f64,97f64,40f64,83f64,13f64,31f64,60f64,37f64,95f64,76f64,8f64,58f64,42f64,11f64,88f64,82f64,20f64,5f64,35f64,20f64,6f64,77f64,42f64,8f64,17f64,59f64,52f64,46f64,22f64,18f64,32f64,43f64,34f64,96f64,81f64,89f64,56f64,51f64,96f64,15f64,61f64,67f64,58f64,16f64,75f64,94f64,99f64,65f64,84f64,41f64,84f64,85f64,15f64,37f64,16f64,91f64,12f64,72f64,43f64,6f64,98f64,67f64,66f64,2f64,6f64,27f64,30f64,67f64,75f64,34f64,4f64,24f64,70f64,49f64,51f64,52f64,14f64,57f64,98f64,11f64,34f64,97f64,100f64,85f64,88f64,48f64,29f64,68f64,83f64,64f64,30f64,12f64,92f64,10f64,38f64,26f64,91f64,59f64,45f64,61f64,93f64,41f64,2f64,37f64,85f64,84f64,7f64,66f64,83f64,10f64,96f64,14f64,61f64,54f64,2f64,93f64,3f64,73f64,75f64,54f64,50f64,10f64,7f64,70f64,10f64,21f64,79f64,97f64,91f64,45f64,41f64,75f64,35f64,8f64,69f64,91f64,46f64,57f64,45f64,35f64,81f64,61f64,69f64,95f64,4f64,20f64,83f64,52f64,19f64,86f64,4f64,85f64,32f64,71f64,95f64,50f64,65f64,59f64,92f64,61f64,35f64,91f64,16f64,98f64,58f64,63f64,40f64,81f64,23f64,87f64,92f64,32f64,30f64,69f64,3f64,55f64,78f64,66f64,62f64,52f64,66f64,77f64,37f64,98f64,60f64,29f64,61f64,24f64,52f64,40f64,64f64,14f64,58f64,98f64,67f64,19f64,48f64,28f64,59f64,80f64,38f64,81f64,25f64,4f64,19f64,40f64,37f64,75f64,85f64,58f64,99f64,73f64,39f64,39f64,39f64,63f64,31f64,24f64,12f64,19f64,23f64,69f64,51f64,12f64,91f64,78f64,68f64,49f64,40f64,50f64,38f64,34f64,11f64,83f64,95f64,63f64,63f64,3f64,23f64,79f64,41f64,12f64,71f64,15f64,86f64,65f64,40f64,2f64,7f64,2f64,8f64,88f64,37f64,32f64,36f64,94f64,64f64,52f64,28f64,24f64,91f64,17f64,4f64,74f64,84f64,98f64,70f64,1f64,80f64,5f64,40f64,80f64,59f64,16f64,32f64,40f64,91f64,87f64,60f64,56f64,96f64,17f64,64f64,76f64,41f64,14f64,38f64,8f64,45f64,71f64,26f64,69f64,94f64,21f64,26f64,76f64,41f64,57f64,66f64,39f64,18f64,74f64,11f64,25f64,62f64,34f64,60f64,74f64,92f64,51f64,3f64,95f64,32f64,64f64,21f64];
	let test: KolmogorovSmirnovTest = KolmogorovSmirnovTest{rnd_values: vector, lower_bound: 0f64, upper_bound: 100f64, num_intervals: 5};
	let result: KolmogorovSmirnovTestResult = test.test();
	assert!(!result.is_random(0.05));
}

