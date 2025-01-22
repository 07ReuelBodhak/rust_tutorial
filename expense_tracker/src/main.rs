use std::{collections::HashMap,io};

#[derive(Debug)]
struct Expense {
    amount : f64,
    description : String,
}

fn main(){
    let mut map:HashMap<String, Vec<Expense>> = HashMap::new();
    loop {
          let mut choice = String::new();
    
    println!("**********Menu**********\n1.Add\n2.View\n3.Update\n4.Delete\n5.exit");
    io::stdin()
        .read_line(&mut choice)
        .expect("unable to read choice");
    let choice = choice.trim();

    match choice {
        "1" => {
            println!("\n********ADD**********\n");
            let mut input = String::new(); 
            let mut category = String::new();
                
            println!("please enter category : ");
            io::stdin()
                .read_line(&mut category)
                .expect("unable to read the description");
            let category = category.trim().to_string();

            println!("please enter description where u spend the money : ");
            io::stdin()
                .read_line(&mut input)
                .expect("unable to read the description");
            
            let description = input.trim().to_string();
            input.clear();
        
            println!("please enter the amount");
                io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let amount:f64 = input
                .trim()
                .parse()
                .expect("please enter valid number");
            
            let expense = Expense {
                amount,
                description
            };

            map.entry(category)
                .or_insert_with(Vec::new)
                .push(expense);
            println!("\n***********************\n");
            
        },
        "2" => {
            println!("\n*********total expenses********\n");
            if map.len() > 0{
                for (key,value) in &map{
                println!("Category - {}",key.trim());
                let mut sum = 0.0;
                for (i,exp) in value.iter().enumerate(){
                    println!("  {}. {} : {}",i+1,exp.amount,exp.description);
                    sum += exp.amount;
                }
                println!("\ntotal money spend on {} : {sum}\n",key.trim());
                }
            }else{
                println!("Nothing Added yet");
            }
            println!("\n*******************************\n");
        },
        "3" => {
            println!("\n********UPDATE**********\n");
            loop {
                let mut input = String::new();
                for (i,(key,_)) in map.iter().enumerate() {
                    println!("{}. {key}",i+1);
                }
                println!("\nplease select the category name you want to update : ");

                io::stdin()
                    .read_line(&mut input)
                    .expect("invalid name");
                let input = input.trim();
                
                if map.contains_key(input){
                    for (i,expense) in map[input].iter().enumerate(){
                        println!("{},{:?}",i+1,expense);
                    }

                    let mut index = String::new();
                    println!("please enter the index number : ");
                    io::stdin()
                        .read_line(&mut index)
                        .expect("unable to read value");
                    
                    let ind:usize = index
                        .trim()
                        .parse()
                        .expect("msg");

                    let ind = ind - 1;

                    let mut choice = String::new();
                    println!("please enter what u want to update");
                    println!("1. description\n2. amount");
                    
                    io::stdin()
                        .read_line(&mut choice)
                        .expect("invalid choice");

                    let choice = choice.trim();

                    match choice {
                        "1" => {
                            println!("new description : ");
                            let mut description = String::new();
                            io::stdin()
                                .read_line(&mut description)
                                .expect("invalid description");
                            
                            let description = description.trim().to_string();
                            map.get_mut(input)
                                .unwrap()
                                .get_mut(ind)
                                .unwrap()
                                .description = description;
                            break;
                        },
                        "2" => {
                            println!("new amount : ");
                            let mut amount = String::new();
                            io::stdin()
                                .read_line(&mut amount)
                                .expect("error");
                            let amount:f64 = amount
                                .trim()
                                .parse()
                                .expect("error");

                            map.get_mut(input)
                                .unwrap()
                                .get_mut(ind)
                                .unwrap()
                                .amount = amount;
                            break;                            
                        }
                        _ => println!("invalid choice")
                    };

                }  else{
                    println!("Category not found");
                    println!("Do you wish to continue?...\ny,n");

                    let mut choice = String::new();
                    io::stdin().read_line(&mut choice).expect("Invalid choice");
                    let choice = choice.trim();

                    match  choice {
                        "y" => continue,
                        "n" => break,
                        _ => println!("invalid choice"),
                    }
                }
            }
            println!("\n**************************\n");
        }
        "4" => {
            println!("\n********DELETE*********\n*");
            for (key,_) in &map{
                println!("{key}");
            }
            println!("please enter the category name u want to delete");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("unable to read value");
            
            let key_to_remove = input
                .trim()
                .to_string();
            let removed_value = map.remove(&key_to_remove);

            match removed_value {
                Some(_) => println!("Removed: {}", key_to_remove),
                None => println!("Key not found: {}", key_to_remove),
            };
            println!("\n*************************\n");
        }
        "5" => {
            break;
        },
        _ => println!("invalid choice"),
        }
    }

}