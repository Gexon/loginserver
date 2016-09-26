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


pub fn get_mdhash(name: &str) -> String {
    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();
    let mut stmt0 = pool.prepare("  SELECT hash \
                                    FROM dotakiller.accounts \
                                    WHERE name=?", ).unwrap();
    for row in stmt0.execute((name,)).unwrap() {
        let retHash: String = mysql::from_row::<String>(row.unwrap());
        return retHash;
    }

    let retHash: String = "".to_string();
    return retHash;
}

pub fn add_account(name: &str, hash: &str, rage911: i32) {
    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();

    let mut stmt0 = pool.prepare("  INSERT \
                                    INTO dotakiller.accounts (name, hash, rage911) \
                                    VALUES (?, ?, ?)").unwrap();
    stmt0.execute((name, hash, rage911)).unwrap();
}

/// Проверяем есть ли пользователь с именем name
pub fn check_name(name: &str) -> bool {
    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();

    let mut stmt0 = pool.prepare(format!("  SELECT * \
                                            FROM dotakiller.accounts \
                                            WHERE name='{}'", name)).unwrap();
    for row in stmt0.execute(()).unwrap() {
        return true;
    }

    false
}
