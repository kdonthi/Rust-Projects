use std::io;

fn earth_to_mars(earth_weight: f32) -> f32 {
    let mass = earth_weight / 9.807;
    let mars_weight: f32 = mass * 3.711;
    return mars_weight;
}
fn main() {
    
    println!("Type your weight in N on Earth: ");
    let mut string: String = String::new(); //what if this was empty?

    io::stdin().
        read_line(&mut string).
        expect("Error in reading");

    let earth_weight: f32 = string.trim().parse::<f32>().expect("Error in reading weight");

    println!("Your weight on Earth is {} Newtons and your weight on Mars is around {:.2} Newtons.", earth_weight, earth_to_mars(earth_weight));
}
