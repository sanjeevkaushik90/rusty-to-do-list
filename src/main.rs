use std::io;

fn main() {
    store_value();
}

fn user_input() -> Result<String, String> {
    let mut input = String::new();

    let user = io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read input".to_string())?;

    Ok(input.trim().to_string())
}

fn store_value() {
    let mut store = Vec::new();

    loop {
        match user_input() {
            Ok(value) => store.push(value),
            Err(e) => println!("{}", e),
        }
        println!("{:?}", store);
        println!("Do you want to countiue(y/n)");
        let mut start = String::new();
        io::stdin().read_line(&mut start).expect("msg");
        let user=start.as_str().trim();

        if user == "n" {
            break;
        }
        else if user=="y"{
            continue;
        }
        else {
            println!("Invalid Choice");
        }
    }
}
