use std::io::Write;

pub fn run() {
    let mut _shareholders: Vec<&str> = vec![];
    let mut _money: Vec<i32> = vec![];
    let mut _perc: Vec<f32> = vec![];
    let mut _valuation: i32 = 0;

    loop {
        let _res: &str = input("/ !> ");
        
        if _res == "add" {
            let mut _calc: Vec<f32> = vec![];

            _shareholders.push(input("/add/ shareholder name ?> "));
            let _res: i32 = input("/add/ investment($) ?> ").parse::<i32>().unwrap();

            _money.push(_res);
            let _roi: f32 = _money[_money.len() - 1] as f32 / (_money[_money.len() - 1] as f32 + _valuation as f32);
            
            _perc.push(_roi * 100.0);

            for _i in 0.._shareholders.len() - 1 {
                _calc.push(_perc[_i] / _perc[_perc.len() - 1]);
            }

            let mut _prev_perc: f32 = 0.0;

            for _i in 0.._calc.len() {
                _prev_perc += _calc[_i];
            }

            for _i in 0.._shareholders.len() - 1 {
                _perc[_i] = _perc[_i] - (_perc[_i] / _prev_perc);
            }

            _valuation += _res;
        }

        else if _res == "show" {
            println!("\n[valuation: ${}]", _valuation);

            for _i in 0.._shareholders.len() {
                println!("[{}   ${}              %{}]", _shareholders[_i], _money[_i], _perc[_i]);
            }

            println!("");
        }
        
        else if _res == "invest" {
            if _shareholders.len() > 0 {
                _valuation += input("/invest/ investment($) ?> ").parse::<i32>().unwrap();
    
                for _i in 0.._shareholders.len() {
                    _money[_i] = (_valuation as f32 * _perc[_i] / 100.0) as i32;
                }

            }

            else {
                println!("!");
            }
        }
        
        else if _res == "ls" {
            println!("\n[show]\n[add]\n[invest]\n[clear]\n");
        }

        else if _res == "clear" {
            if input("/clear/ clear all shareholders [y/n] ?> ") == "y" {
                
                _shareholders.clear();
                _money.clear();
                _perc.clear();
                _valuation = 0;
            }
            
        }

        else {
            println!("!");
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