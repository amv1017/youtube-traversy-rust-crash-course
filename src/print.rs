pub fn run() {
  // Print to console
  println!("Hello from the print.rs file!");

  // Basic Formatting
  println!("{} is from {}", "Brad", "Massachusetts");

  // Positional Arguments
  println!("{0} is from {1} and {0} likes to {2}",
  "Brad", "Massachusetts", "code");

  // Named Arguments
  println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

  // Placeholder Traits
  println!("Binary: {:b} \nHex: {:x} \nOctal: {:o}", 10,10,10);

  // Placeholder For Debug Traits
  println!("{:?}",(12, true, "Hello"));

  // Basic Math
  println!("7*8 = {}",7*8)
}
