use std::{io::{self}};

fn main() {
    store_value();

    let task=Task{
        title:String::from("Write a report"),
        completed:false,

    };
    println!("{}",task.title);
}

struct Task {
    title: String,
    completed: bool,
}

fn user_input() -> Result<Task, String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read input".to_string())?;

    Ok(Task {
    title: input.trim().to_string(),
    completed: false,
})
}

fn store_value() {
    let mut store:Vec<Task> = Vec::new();

    loop {
        match user_input() {
            Ok(task) => store.push(task),
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
        println!("{} : {}", index + 1, value.title)
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
            println!("{} : {}", index + 1, value.title)
        }
    }
    else if wish=="n" {
        break;
    }
}
}
