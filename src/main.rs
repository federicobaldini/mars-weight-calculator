use std::io;

fn main() {
  let weight_on_earth;
  let weight_on_mars;

  weight_on_earth = get_input_weight_on_earth();
  if weight_on_earth > 0.0 {
    weight_on_mars = calculate_weight_on_mars(weight_on_earth);
    println!("{}(kg) on Earth are equivalent to {}(kg) on Mars", weight_on_earth, weight_on_mars);
  }
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
  (weight_on_earth / 9.81) * 3.711
}

fn get_input_weight_on_earth() -> f32 {
  let mut input = String::new();

  println!("Enter a weight (kg): ");
  io::stdin().read_line(&mut input).unwrap();

  return input.trim().parse::<f32>().unwrap();
}