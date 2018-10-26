use std::io;
use std::io::Write;

fn main() {
    enum ConversionType {
        None,
        DecToBase,
        BaseToDec
    };

    'main: loop {
        let mut input = String::new();
        let mut c_type = ConversionType::None;

        println!("*** Rust Base Converter ***\n\n");

        while let ConversionType::None = c_type {
            input = String::new();

            println!("(D)ec to Base");
            println!("(B)ase to Dec");
            println!("(Q)uit");
            print!("\n--> ");

            io::stdout().flush().expect("Error flushing stdout");
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().as_ref() {
                "d" | "D" => {
                    println!("\n\nConverting Decimal to Base\n");
                    c_type = ConversionType::DecToBase;
                },
                "b" | "B" => {
                    println!("\n\nConverting Base to Decimal\n");
                    c_type = ConversionType::BaseToDec;
                },
                "q" | "Q" => {
                    println!("Bye!");
                    break 'main;
                }, // Exit the program
                _ => {
                    println!("{} was not a choice. Please enter one of the provided choices\n\n", input.trim());
                    continue; // Bail and loop again if the choice is not valid
                },
            }

            input = String::new();
        }

        while let Err(_err) = input.trim().parse::<u32>() {
            input = String::new();

            // Get the Base
            println!("Please enter a base");
            print!("\n--> ");

            io::stdout().flush().expect("Error flushing stdout");
            io::stdin().read_line(&mut input).unwrap();

            // Trim off trailing newline
            let l = input.len();
            input.truncate(l - 1);

            if !input.trim().parse::<u32>().is_ok() {
                println!("Base must be a positive integer");
            }
        }

        let base: u32 = input.trim().parse::<u32>().expect("Base should be a positive integer");

        let mut num: Option<String> = None;

        while let None = num {
            input = String::new();

            // Get the number
            println!("Please enter a number");
            print!("\n--> ");

            io::stdout().flush().expect("Error flushing stdout");
            io::stdin().read_line(&mut input).unwrap();

            // Trim off trailing newline
            let l = input.len();
            input.truncate(l - 1);

            if let ConversionType::DecToBase = c_type {
                if input.trim().parse::<u32>().is_ok() { 
                    num = Some(String::from(input.trim()));
                } else { 
                    num = None;

                    println!("{} is not a number. Decimals must be numeric.", input.trim());
                }
            } else {
                num = Some(String::from(input.trim()));
            }
        }

        match c_type { 
            ConversionType::None => {
                break 'main;
            },
            ConversionType::BaseToDec => {
                let num_val = num.expect("Number has no value");

                let conv = convert_base_to_dec(&num_val, &base);

                println!("{} (base {}) in decimal is {}", num_val, base, conv);
            },
            ConversionType::DecToBase => { 
                let dec = num.expect("Number has no value") // unwrap num
                                .parse::<u32>().expect("Error converting number to int"); // convert num to i3

                let conv = convert_dec_to_base(&dec, &base);

                println!("{} in base {} is {}", dec, base, conv);
            }
        }
    }
}

fn convert_dec_to_base(dec: &u32, base: &u32) -> String {
    String::from("")
}


/**
*	 Let num = string rep of starting number
*	 Let base = base to convert from
*	 Let numLen = length of num
*	 
*	 While numLen > 0:
*		first = leftmost char in num
*		num = num excluding first
*		check if base > 10 and make any necessary conversions
*		convert first to an int
*		decrement numLen
*		add first * (base^numLen) to result
**/
fn convert_base_to_dec(num: &String, base: &u32) -> u32 {
    let mut num_vec: Vec<char> = num.chars().collect();
    let mut num_len = num.len();
    let mut result = 0;

    while num_len > 0 {
        let first = num_vec.first().cloned().unwrap();
        let first_int: u32;

        num_vec.remove(0);
        
        match first {
            'A' | 'a' => {
                first_int = 10;
            },
            'B' | 'b' => {
                first_int = 11;
            },
            'C' | 'c' => {
                first_int = 12;
            },
            'D' | 'd' => {
                first_int = 13;
            },
            'E' | 'e' => {
                first_int = 14;
            },
            'F' | 'f' => {
                first_int = 15;
            },
            _ => first_int = first.to_digit(10).unwrap() // convert first to u32
        }

        if (first_int >= *base && *base > 1) || (*base == 1 && first_int > 1) {
            println!("\nCould not finish conversion: num cannot be >= than base");

            return 0;
        }

        num_len -= 1;
        result += first_int * (base.pow(num_len as u32));
    }

    result
}