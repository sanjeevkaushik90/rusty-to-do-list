use std::io::{self};

fn main() {
    store_value();
}

fn user_input() -> Result<String, String> {
    let mut input = String::new();

    io::stdin()
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

        println!("Do you want to countiue(y/n)");
        let mut start = String::new();
        io::stdin().read_line(&mut start).expect("msg");
        let user = start.as_str().trim();

        if user == "n" {
            break;
        } else if user == "y" {
            continue;
        } else {
            println!("Invalid Choice");
        }
    }
    println!("Current Task:");
    for (index, value) in store.iter().enumerate() {
        println!("{} : {}", index + 1, value)
    }
loop{
    println!("Do you wish to do delete any task(y/n)");

    let mut start = String::new();
    io::stdin().read_line(&mut start).expect("msg");
    let wish = start.as_str().trim();

    if wish == "y" {
        println!("Choose number of task (e.g. 1,2,3,4):");

        let mut index = String::new();
        io::stdin().read_line(&mut index).expect("msg");

        let number: u32 = match index.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("enter a vaild number");
                return;
            }
        };


        if number>=1 &&  number <= store.len().try_into().unwrap() {

            let index = number as usize - 1;
            store.remove(index);
        }
        else{
            println!("Invalid task number")
        }

        println!("Current Task:");
        for (index, value) in store.iter().enumerate() {
            println!("{} : {}", index + 1, value)
        }
    }
    else if wish=="n" {
        break;
    }
}
}

