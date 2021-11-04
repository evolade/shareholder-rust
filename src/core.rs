use std::io::Write;

pub fn run() {
    let mut _shareholders: Vec<&str> = vec![];
    let mut _money: Vec<i32> = vec![];
    let mut _perc: Vec<f32> = vec![];
    let mut _valuation: i32 = 0;
    
    loop {
        let res: &str = input("/ !> ");
        
        if res == "add" {
            let mut _calc: Vec<f32> = vec![];

            _shareholders.push(input("/add!holder/ \"shareholder name\" !> "));
            let res: i32 = input("/add!holder/ \"investment($)\" !> ").parse::<i32>().unwrap();

            _money.push(res);
            let _roi: f32 = _money[_money.len() - 1] as f32 / (_money[_money.len() - 1] as f32 + _valuation as f32);
            
            _perc.push(_roi * 100.0);

            for i in 0.._shareholders.len() - 1 {
                _calc.push(_perc[i] / _perc[_perc.len() - 1]);
            }

            let mut kl: f32 = 0.0;

            for i in 0.._calc.len() {
                kl += _calc[i];
            }

            for i in 0.._shareholders.len() - 1 {
                _perc[i] = _perc[i] - (_perc[i] / kl);
            }
        }

        else if res == "show" {
            println!("\n[valuation: ${}]", _valuation);

            for i in 0.._shareholders.len() {
                println!("[{}   ${}              %{}]", _shareholders[i], _money[i], _perc[i]);
            }

            println!("");
        }
        
        else if res == "invest" {
            _valuation += input("/invest/ \"investment($)\" !> ").parse::<i32>().unwrap();

            for i in 0.._shareholders.len() {
                _money[i] = (_valuation as f32 * _perc[i] / 100.0) as i32;
            }
        }
        
        else if res == "ls" {
            println!("\n[show]\n[add]\n[invest]\n");
        }

        else {
            println!("!404!");
        }
    }
}

// rust doesnt have a solid input system so I created one
fn input(headline: &str) -> &str{
    print!("{}", headline);
    std::io::stdout().flush().unwrap(); // idk what this thing does dont remove it
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).expect("err"); // take input
    _input.pop(); // pop the last index ("\n")
    let _str_input: &str = Box::leak(_input.into_boxed_str()); // convert String to &str
    return _str_input;
}