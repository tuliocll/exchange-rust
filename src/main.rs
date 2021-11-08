use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "435d4e9f9bed96d21907";

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("Erro! Faltando argumentos");
    }

    let from = &args[1];
    let to = &args[2];
    let value: f32 = args[3].parse().expect("Digite um número valido!");

    let convert: String = format!("{}_{}", from, to);
    let response = reqwest::get(format!(
        "https://free.currconv.com/api/v7/convert?q={}&compact=ultra&apiKey={}",
        convert, api_key
    ))
    .await?
    .json::<HashMap<String, f32>>()
    .await?;

    let returned_value = response[&convert.to_uppercase()];

    let final_value = value * returned_value;

    println!("{:#?} {}", final_value, to.to_uppercase());

    Ok(())
}
