use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct EnvTest{
    thing1: String,
    thing2: i32
}

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    let mut val = envy::from_env::<EnvTest>().unwrap();
    println!("{}", val.thing1)

}
