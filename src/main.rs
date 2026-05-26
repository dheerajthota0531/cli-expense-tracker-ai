use csv;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io,
};

use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Record {
    amount:f64,
    category: String,
    description: String,

}

fn main() {
    println!("Welcome To Expense Tracker-Pro");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("falied to read");
        let input = input.trim();

        match input {
            "add" => {
                println!("Adding Boss...");
                 if let Err(e) = add_expenses(){
                    eprintln!("{:?}",e)
                 }
                
            },
            "list" => {
                println!("Listing Boss..");
              if let Err(e) = list_expenses(){
                    eprintln!("{:?}",e)
                 }
            }
            "quit" => {
                println!("GoodBye.. ");
                break;
            },
            "sum" => {
                println!("presenting you the summary");
                if let Err(e) =summary(){
                    eprintln!("{:?}", e);
                }
            }
            _ => println!("add or list"),
        }
    }
}

fn summary() -> Result<(), Box<dyn Error>>{
    let mut sum:HashMap< String, f64 > = HashMap::new();
    let file = File::open("expenses.csv")?;
    let mut  rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record:Record = result?;
        let total = sum.entry(record.category).or_insert(0.0);
         *total += record.amount;
    }


    for (category, amount) in sum {
        println!("{} => {}", category, amount);
        
    }
    
    Ok(())
}


fn list_expenses() -> Result<(), Box<dyn Error>> {
    let file = File::open("expenses.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record:Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}


fn add_expenses() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().append(true).create(true).open("expenses.csv")?;
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_writer(file);

    let mut  expense_amout = String::new();
    println!("Enter amout");
    io::stdin().read_line(&mut expense_amout).expect("failed to read input");
    let amt:f64 = expense_amout.trim().parse()?;

    let mut  expense_categroy = String::new();
    println!("Enter category");
    io::stdin().read_line(&mut expense_categroy).expect("failed to read input");
    let cate = expense_categroy.trim().to_string();

    let mut  expense_description = String::new();
    println!("Enter description");
    io::stdin().read_line(&mut expense_description).expect("failed to read input");
    let desc = expense_description.trim().to_string();

    let  new_expense = Record{
        amount:amt,
        category:cate,
        description:desc,
    };

    wtr.serialize(new_expense)?;
    wtr.flush()?;
    
    Ok(())
}