use clap::Parser;
use std::io::{self, Write};

#[derive(Parser)]
struct Cli {
    /// O numero desejado para converter
    #[arg(short, long)]
    numero: Option<f64>,

    /// Se quer converter de binario para decimal
    #[arg(short, long)]
    decimal: bool,
}

macro_rules! input {
    ($prompt:literal) => {{
        print!($prompt);
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.retain(|c| !c.is_whitespace()); // fastest way to remove whitespace without allocating a new String
        input
    }};
}

fn get_cli_args() -> (f64, bool) {
    let args = Cli::parse();
    let numero = if let Some(num) = args.numero {
        num
    } else {
        match input!("Numero: ").parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("O numero inserido não é valido!");
                std::process::exit(1);
            }
        }
    };

    (numero, args.decimal)
}

fn decimal_to_binary(decimal: f64) -> String {
    let integer_part = decimal.trunc() as i64;
    let fractional_part = decimal - integer_part as f64;

    let integer_binary = format!("{:b}", integer_part);

    let mut fractional_binary = String::new();
    let mut fractional_part = fractional_part;

    while fractional_part > 0.0 {
        fractional_part *= 2.0;
        if fractional_part >= 1.0 {
            fractional_binary.push('1');
            fractional_part -= 1.0;
        } else {
            fractional_binary.push('0');
        }
    }

    if !fractional_binary.is_empty() {
        format!("{}.{}", integer_binary, fractional_binary)
    } else {
        integer_binary
    }
}

fn binary_to_decimal(binario: f64) -> String {
    let integer_part = binario.trunc() as i64;

    let fractional_part = binario - integer_part as f64;

    let decimal = isize::from_str_radix(&integer_part.to_string(), 2).unwrap();

    let mut result = 0.0;

    if fractional_part != 0.0 {
        let fractional_part_length = binario.to_string().len() - integer_part.to_string().len() - 1;

        let mult = 10.0_f64.powi(fractional_part_length as i32);
        let fractional_part = (fractional_part * mult).round() / mult;

        let fractional_string = fractional_part.to_string();
        let fractional_str = &fractional_string[2..fractional_string.len()].to_string();

        if fractional_string != "0" {
            for i in 0..fractional_str.len() {
                let power = 2.0_f64.powi((i + 1) as i32 * -1);
                result += power;
            }
        }
    }
    result += decimal as f64;

    result.to_string()
}

fn main() {
    let (numero, decimal) = get_cli_args();

    if decimal {
        let decimal = binary_to_decimal(numero);
        println!("{}", decimal);
        return;
    }
    let binario = decimal_to_binary(numero);

    println!("{}", binario);
}
