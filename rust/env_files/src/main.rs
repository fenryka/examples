use serde::Deserialize;

#[derive(Deserialize, Default)]
struct EnvTest{
    thing1: String,
    thing2: i32,
    thing3: String,
    thing4: i64
}

fn main() {

    /*
    // Naive
    let val = envy::from_env::<EnvTest>().unwrap();
    println!("{} {} {}", val.thing1, val.thing2, val.thing3)
     */

    dotenv::dotenv().expect("Failed to read .env file");
    let val = envy::from_env::<EnvTest>().unwrap_or_else(|error|{
        println!("Failed to parse env-vars");
        return Default::default();
    });
    println!("'{}' '{}' '{}' '{}'", val.thing1, val.thing2, val.thing3, val.thing4)
}
