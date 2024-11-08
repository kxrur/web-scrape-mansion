use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq)] // can't use decimal point numbers with 'Eq'
pub struct Mansion {
    pub address1: String,
    pub address2: Option<String>, //allows for None address2
    pub price: i32,               // FIXME: fix sql table to have INT price (currently f64)
    pub size: f64,
    pub bedrooms: i32,
    pub bathrooms: i32, //TODO: add a receptions field
    pub mansion_type: String,
}

pub fn establish_pool() -> Result<Pool, mysql::Error> {
    let url = OptsBuilder::new()
        .user(Some(dotenv!("MYSQL_USER")))
        .pass(Some(dotenv!("MYSQL_PASSWORD")))
        .ip_or_hostname(Some(dotenv!("MYSQL_ADDRESS")))
        .db_name(Some(dotenv!("MYSQL_DATABASE")));
    // let pool = match Pool::new(url) {
    //     Ok(it) => it,
    //     Err(err) => return Err(Box::new(err)),
    // };

    Pool::new(url)
}

pub fn some_mansions() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = establish_pool()?;

    let mansions = vec![
        Mansion {
            address1: "123 Maple St".to_string(),
            address2: Some("Apt 1A".to_string()),
            price: 15,
            size: 25.0,
            bedrooms: 4,
            bathrooms: 5,
            mansion_type: "Villa".to_string(),
        },
        Mansion {
            address1: "456 Oak Ave".to_string(),
            address2: None,
            price: 20,
            size: 30.5,
            bedrooms: 5,
            bathrooms: 6,
            mansion_type: "Estate".to_string(),
        },
        Mansion {
            address1: "789 Pine Rd".to_string(),
            address2: Some("Penthouse".to_string()),
            price: 22,
            size: 35.0,
            bedrooms: 3,
            bathrooms: 4,
            mansion_type: "Penthouse".to_string(),
        },
        Mansion {
            address1: "101 Elm St".to_string(),
            address2: None,
            price: 18,
            size: 28.0,
            bedrooms: 4,
            bathrooms: 4,
            mansion_type: "Mansion".to_string(),
        },
    ];

    let _ = push(&mansions, &pool);

    let selected_mansions = match pull(&pool) {
        Ok(it) => it,
        Err(err) => return Err(Box::new(err)),
    };

    assert_eq!(dbg!(mansions), dbg!(selected_mansions));
    println!("Yay!");

    Ok(())
}

pub fn push(
    mansions: &Vec<Mansion>,
    pool: &Pool,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut conn = match pool.get_conn() {
        Ok(it) => it,
        Err(err) => return Err(Box::new(err)),
    };
    match conn.exec_batch(
        r"INSERT INTO mansions (address1, address2, price, size, bedrooms, bathrooms, type)
      VALUES (:address1, :address2, :price, :size, :bedrooms, :bathrooms, :type)",
        mansions.iter().map(|m| {
            params! {
                "address1" => &m.address1,
                "address2" => &m.address2,
                "price" => m.price,
                "size" => m.size,
                "bedrooms" => m.bedrooms,
                "bathrooms" => m.bathrooms,
                "type" => &m.mansion_type
            }
        }),
    ) {
        Ok(_) => println!("Insert successful"),
        Err(e) => eprintln!("Insert failed: {:?}", e),
    }
    Ok(())
}

pub fn pull(pool: &Pool) -> Result<Vec<Mansion>, mysql::Error> {
    let mut conn = match pool.get_conn() {
        Ok(it) => it,
        Err(err) => return Err(err),
    };

    conn.query_map(
        "SELECT address1, address2, price, size, bedrooms, bathrooms, type FROM mansions",
        |(address1, address2, price, size, bedrooms, bathrooms, mansion_type)| Mansion {
            address1,
            address2,
            price,
            size,
            bedrooms,
            bathrooms,
            mansion_type,
        },
    )
}
