use csv;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io,
};

use std::collections::HashMap;
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
enum RecordType{
    
    Income,
    Expense,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Record {
     #[serde(rename = "type")]
    spend_type:RecordType,
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
    let mut total_income = 0.0;
    let mut total_expense = 0.0;

    let mut  rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record:Record = result?;
        match record.spend_type {
            RecordType::Expense => {
                total_expense += record.amount;
            }
            RecordType::Income => {
                total_income += record.amount;

                let total = sum.entry(record.category).or_insert(0.0);
                *total += record.amount;
            }
        } 
    }
    println!("{}", "++".repeat(30));
    let net_bal = total_income - total_expense;
        
    println!("Total Income   => {:.2}", total_income);
    println!("Total Expense  => {:.2}", total_expense);
    println!("Net Balance    => {:.2}", net_bal);

     println!("{}", "++".repeat(15));


    if net_bal < 0.0 {
        println!("Status         => Deficit");
    } else {
        println!("Status         => surplus");
    }

     println!("{} Category {}", "++".repeat(15),  "++".repeat(15));
    for (category, amount) in sum {
        println!("{} => {}", category, amount);
        
    }

    println!("{}", "++".repeat(30));
    
    Ok(())
}


fn list_expenses() -> Result<(), Box<dyn Error>> {
    let file = File::open("expenses.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    println!("\n{:<10} {:<10} {:<15} ", "Type, Amount", "Category", "Description");
    let mut total = 0.0;
    let mut count = 0;
    println!("{}", "-".repeat(50));
    for result in rdr.deserialize() {
        let record:Record = result?;
       println!("{:?} {:<10.2} {:<15} {}", record.spend_type,record.amount, record.category, record.description);
       total += record.amount;
       count += 1;
    }

    if count == 0{
        println!("No Expenses");
    }else{
        println!("{}", "-".repeat(50));
        println!("{:<10.2} ({count} expenses)", total);
    }

    Ok(())
}


fn add_expenses() -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().append(true).create(true).open("expenses.csv")?;
    let mut wtr = csv::WriterBuilder::new()

        .delimiter(b',')
        .has_headers(false)
        .from_writer(file);

    let mut spend = String::new();
    println!("Enter Type: Income/Expense");
    io::stdin().read_line(&mut spend).expect("failed to read input");
    let spend_type = match spend.trim() {
        "Income" => RecordType::Income,
        "Expense" => RecordType::Expense,
        _ => {
            println!("Choose: Income or Expense");
            return Ok(())
        }
    };


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
        spend_type,
        amount:amt,
        category:cate,
        description:desc,
    };

    wtr.serialize(new_expense)?;
    wtr.flush()?;
   
    println!("Added successfully" );
    
    Ok(())
}
