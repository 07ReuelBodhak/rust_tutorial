use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use chrono::Local;

#[derive(Debug)]
struct Account {
    balance: f64,
    history: Vec<Transaction>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Transaction {
    transaction_type: String,
    amount: f64,
    time_stamp: String,
}

fn main() {
    let mut map: HashMap<String, Account> = HashMap::new();

    loop {
        println!("************ ACCOUNT SYSTEM *************");
        println!("1. Create new Account");
        println!("2. Log in to your Account");
        println!("3. Exit");
        println!("***************************************");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Unable to read choice");

        match choice.trim() {
            "1" => create_account(&mut map),
            "2" => login(&mut map),
            "3" => break,
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

// Function to create an account
fn create_account(map: &mut HashMap<String, Account>) {
    println!("************ CREATE ACCOUNT *************");
    println!("Please enter a new Account number: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read account name");
    let account_name = input.trim().to_string();

    if map.contains_key(&account_name) {
        println!("Account already exists!");
        return;
    }

    let account = Account {
        balance: 0.0,
        history: Vec::new(),
    };
    map.insert(account_name.clone(), account);

    let file_path = format!("{account_name}.txt");
    if let Err(error) = File::create(&file_path) {
        panic!("Unable to create account file: {}", error);
    }

    println!("Account '{}' created successfully!", account_name);
}

// Function to log in
fn login(map: &mut HashMap<String, Account>) {
    println!("Enter your account number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Invalid account name");

    let account_name = input.trim().to_string();
    if !map.contains_key(&account_name) {
        println!("Account '{}' does not exist!", account_name);
        return;
    }

    let file_path = format!("{account_name}.txt");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)
        .expect("Failed to open or create the file");

    loop {
        println!("\n*********** BANK OF CODERS ***************");
        println!("1. View Balance");
        println!("2. Deposit");
        println!("3. Withdraw");
        println!("4. Transaction History");
        println!("5. Exit");
        println!("******************************************");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Invalid choice");

        match choice.trim() {
            "1" => {
                println!("\n----------- Balance -----------");
                println!("Balance: ${:.2}", view_balance(map, &account_name));
                println!("-------------------------------");
            }
            "2" => {
                println!("\n----------- Deposit -----------");
                deposit(map, &account_name, &mut file);
                println!("-------------------------------");
            }
            "3" => {
                println!("\n----------- Withdraw -----------");
                withdraw(map, &account_name, &mut file);
                println!("-------------------------------");
            }
            "4" => {
                println!("\n----------- Transaction History -----------");
                view_transaction_history(map, &account_name);
                println!("-------------------------------------------");
            }
            "5" => break,
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

// Function to view balance
fn view_balance(map: &HashMap<String, Account>, account_name: &str) -> f64 {
    map.get(account_name).map_or(0.0, |account| account.balance)
}

fn deposit(map:&mut HashMap<String,Account>,account_name:&str,file:&mut File){
    let mut input = String::new();
    println!("enter amount you to deposit in your account");

    let now = Local::now();

    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    
    io::stdin()
        .read_line(&mut input)
        .expect("invalid input");
    
    let input = input.trim();
    let amount:Result<f64,_> = input.parse();

    match amount {
        Ok(amt) => {
            if let Some(account) = map.get_mut(account_name) {
                let transaction = Transaction {
                    amount : amt,
                    transaction_type : "Deposit".to_string(),
                    time_stamp : formatted_time,
                };
                account.balance += amt;
                account.history.push(transaction);
                println!(
                    "Successfully deposited ${:.2}. New balance: ${:.2}",
                    amt, account.balance
                );
                if let Err(_) = file.set_len(0) {
                    panic!("Unable to truncate the file");
                }
                if let Err(_) = writeln!(file, "{:?}", map) {
                    panic!("Unable to save data to the file");
                }
            }else{
                println!("Account '{}' not found.", account_name);
            }
        },
        Err(_) => panic!("invalid amount"),
    };
}

//fn to withdraw money
fn withdraw(map: &mut HashMap<String, Account>, account_name: &str, file: &mut File) {
    let mut input = String::new();
    println!("Enter the amount you want to withdraw from your account:");

    let now = Local::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    let input = input.trim();
    let amount: Result<f64, _> = input.parse();

    match amount {
        Ok(amt) => {
            if amt <= 0.0 {
                println!("Invalid amount. Please enter a positive number.");
                return;
            }

            if let Some(account) = map.get_mut(account_name) {
                if account.balance >= amt {
                    let transaction = Transaction {
                        amount: amt,
                        transaction_type: "Withdraw".to_string(),
                        time_stamp: formatted_time,
                    };
                    account.balance -= amt;
                    account.history.push(transaction);
                    println!(
                        "Successfully withdrew ${:.2}. New balance: ${:.2}",
                        amt, account.balance
                    );
                    if let Err(_) = file.set_len(0) {
                        panic!("Unable to truncate the file");
                    }
                    if let Err(_) = writeln!(file, "{:?}", map) {
                        panic!("Unable to save data to the file");
                    }
                } else {
                    println!("Insufficient balance. Your current balance is ${:.2}.", account.balance);
                }
            } else {
                println!("Account '{}' not found.", account_name);
            }
        }
        Err(_) => println!("Invalid amount entered. Please enter a valid number."),
    };
}

// Function to view transaction history (placeholder)
fn view_transaction_history(map: &HashMap<String, Account>, account_name: &str) {
    if let Some(account) = map.get(account_name) {
        println!("*********Transaction History**********");
        for his in account.history.iter() {
            println!("Transaction Type : {}",his.transaction_type);
            println!("Amount : {}",his.amount);
            println!("Time : {}",his.time_stamp);
            println!();
        }
    } else {
        println!("Account '{}' not found!", account_name);
    }
}

