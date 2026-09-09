use std::io::{self};

fn main() {
    let mut store: Vec<Task> = Vec::new();

    loop {
        println!("====TO-DO-LIST====");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Delete task");
        println!("4. Complete task");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("msg");

        let num = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                return;
            }
        };

        match num {
            1 => store_value(&mut store),
            2 => list_task(&store),
            3 => delete_task(&mut store),
            4 => compelete_task(&mut store),
            5 => {
                break;
            }

            _ => {
                println!("Invalid choice");
            }
        }
    }
}

#[derive(Debug)]
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

fn store_value(store: &mut Vec<Task>) {
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
        let stauts;

        if value.completed {
            stauts = "Done"
        } else {
            stauts = "Pending"
        }
        println!("{} : {} ->  {}", index + 1, value.title, stauts);
    }
}

fn delete_task(store: &mut Vec<Task>) {
    loop {
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

            if number >= 1 && number <= store.len().try_into().unwrap() {
                let index = number as usize - 1;
                store.remove(index);
            } else {
                println!("Invalid task number")
            }

            println!("Current Task:");
            for (index, value) in store.iter().enumerate() {
                let stauts;

                if value.completed {
                    stauts = "Done"
                } else {
                    stauts = "Pending"
                }
                println!("{} : {} ->  {}", index + 1, value.title, stauts);
            }
        } else if wish == "n" {
            break;
        }
    }
}

fn compelete_task(store: &mut Vec<Task>) {
    println!("Do you want to compelete any task?(y/n)");
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

        if number >= 1 && number <= store.len().try_into().unwrap() {
            let index = number as usize - 1;
            //enter this task is complete
            store[index].completed = true;
        } else {
            println!("Invalid task number")
        }
    }

    println!("Current Task:");
    for (index, value) in store.iter_mut().enumerate() {
        let stauts;

        if value.completed {
            stauts = "Done"
        } else {
            stauts = "Pending"
        }
        println!("{} : {} ->  {}", index + 1, value.title, stauts);
    }
}

fn list_task(store: &Vec<Task>) {
    {
        for (index, value) in store.iter().enumerate() {
            let status;
            if value.completed {
                status = "Done"
            } else {
                status = "Pending"
            }
            println!("{} : {} --> {}", index + 1, value.title, status);
        }
    }
}
