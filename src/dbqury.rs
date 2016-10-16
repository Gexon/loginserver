extern crate mysql;
//use time;

#[derive(Debug, PartialEq, Eq)]
struct Account {
    account_id: i32,
    account_name: String,
    account_hash: String,
}

pub fn _init() {
    //let pool = mysql::Pool::new("mysql://dotakiller:dk@localhost:3307").unwrap();
}


pub fn get_mdhash(name: &str) -> String {
    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();
    let mut stmt0 = pool.prepare("  SELECT hash \
                                    FROM dotakiller.accounts \
                                    WHERE name=?", ).unwrap();
    for row in stmt0.execute((name, )).unwrap() {
        let ret_hash: String = mysql::from_row::<String>(row.unwrap());
        return ret_hash;
    }

    "".to_string()
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

//    for row in stmt0.execute(()).unwrap() {
//        return true;
//    }

    let result = stmt0.execute(());
    let first = result.unwrap().next();
    first.is_some()

}

/// записываем токен в БД
pub fn set_token(name: &str, token: i64){
    //let current_time = time::get_time();
    //let localtime = time::now();
    //let localtime = localtime.asctime();
    //println!("Unixtime: {}, localtime: {}", current_time.sec, localtime);

    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();
    let mut stmt0 = pool.prepare("  UPDATE dotakiller.accounts \
                                    SET token=? \
                                    WHERE name=?", ).unwrap();
    stmt0.execute((token, name)).unwrap();

}

/// получаем токен из БД
pub fn _get_token(name: &str) -> i64{
    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();
    let mut stmt0 = pool.prepare("  SELECT token \
                                    FROM dotakiller.accounts \
                                    WHERE name=?", ).unwrap();
    for row in stmt0.execute((name, )).unwrap() {
        let ret_token: i64 = mysql::from_row::<i64>(row.unwrap());
        return ret_token;
    }

    0

}
