use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Convert binaly, decimal and hexadecimal.", long_about = None)]
pub struct App {
    /// Allow 2, 10, 16
    #[arg(short, long, value_name = "TARGET")]
    target: Option<u32>,

    /// Allow 2, 10, 16
    #[arg(short, long, value_name = "BASE")]
    #[arg(default_value_t = 10)]
    base: u32,

    /// Before conversion value
    value: String,
}

impl App {
    pub fn run() {
        let app = App::parse();
        let decimal = i64::from_str_radix(&app.value, app.base).expect("Invalid value");

        if let None = app.target {
            println!("2:  {}", binaly(decimal));
            println!("10: {}", decimal);
            println!("16: {}", hexadecimal(decimal));
            return;
        }

        match app.target.unwrap() {
            2 => println!("{}", binaly(decimal)),
            10 => println!("{}", decimal),
            16 => println!("{}", hexadecimal(decimal)),
            _ => panic!("Invalid target"),
        }
    }
}

fn binaly(decimal: i64) -> String {
    let mut bin = String::new();
    let mut decimal = decimal;
    while decimal > 0 {
        bin.push_str(&(decimal % 2).to_string());
        decimal /= 2;
    }
    bin.chars().rev().collect()
}

fn hexadecimal(decimal: i64) -> String {
    let mut bin = String::new();
    let mut decimal = decimal;
    let map: Vec<String> = vec!["A", "B", "C", "D", "E", "F"].into_iter().map(|v| String::from(v)).collect();
    while decimal > 0 {
        let devided = decimal % 16;
        if 10 > devided {
            bin.push_str(&devided.to_string());
        } else {
            bin.push_str(&map[(devided - 10) as usize]);
        }
        decimal /= 16;
    }
    bin.chars().rev().collect()
}
