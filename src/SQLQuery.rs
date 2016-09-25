extern crate mysql;

#[derive(Debug, PartialEq, Eq)]
struct Account {
    account_id: i32,
    account_name: String,
    account_hash: String,
}

pub fn init() {
    let pool = mysql::Pool::new("mysql://dotakiller:dk@localhost:3307").unwrap();
}


pub fn get_mdhash(name: &str) {
    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();
    let users: Vec<Account> = pool.prep_exec("SELECT name, hash FROM accounts limit 1000", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (name, hash) = mysql::from_row(row);
                Account {
                    account_id: 0,
                    account_name: name,
                    account_hash: hash
                }
            }).collect()
        }).unwrap();
}

pub fn add_account(name: &str, hash: &str, rage911: i32) {
    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();

    let mut stmt0 = pool.prepare("INSERT INTO dotakiller.accounts (name, hash, rage911) VALUES (?, ?, ?)").unwrap();
    stmt0.execute((name, hash, rage911)).unwrap();
}

