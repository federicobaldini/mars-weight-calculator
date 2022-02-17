fn main() {
  let weight_on_earth = 100.0;
  let weight_on_mars = calculate_weight_on_mars(weight_on_earth);
  println!("{}kg on Earth are equivalent to {}kg on Mars", weight_on_earth, weight_on_mars);
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
  (weight_on_earth / 9.81) * 3.711
}