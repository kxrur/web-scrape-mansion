use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq)] // can't use decimal point numbers with 'Eq'
struct Mansion {
    address1: String,
    address2: Option<String>, //allows for None address2
    price: f64,
    size: f64,
    bedrooms: i32,
    bathrooms: i32,
    mansion_type: String,
}

pub fn some_mansions() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = OptsBuilder::new()
        .user(Some(dotenv!("MYSQL_USER")))
        .pass(Some(dotenv!("MYSQL_PASSWORD")))
        .ip_or_hostname(Some(dotenv!("MYSQL_ADDRESS")))
        .db_name(Some(dotenv!("MYSQL_DATABASE")));
    let pool = match Pool::new(url) {
        Ok(it) => it,
        Err(err) => return Err(Box::new(err)),
    };

    let mut conn = match pool.get_conn() {
        Ok(it) => it,
        Err(err) => return Err(Box::new(err)),
    };

    let mansions = vec![
        Mansion {
            address1: "123 Maple St".to_string(),
            address2: Some("Apt 1A".to_string()),
            price: 15.0,
            size: 25.0,
            bedrooms: 4,
            bathrooms: 5,
            mansion_type: "Villa".to_string(),
        },
        Mansion {
            address1: "456 Oak Ave".to_string(),
            address2: None,
            price: 20.0,
            size: 30.5,
            bedrooms: 5,
            bathrooms: 6,
            mansion_type: "Estate".to_string(),
        },
        Mansion {
            address1: "789 Pine Rd".to_string(),
            address2: Some("Penthouse".to_string()),
            price: 22.0,
            size: 35.0,
            bedrooms: 3,
            bathrooms: 4,
            mansion_type: "Penthouse".to_string(),
        },
        Mansion {
            address1: "101 Elm St".to_string(),
            address2: None,
            price: 18.0,
            size: 28.0,
            bedrooms: 4,
            bathrooms: 4,
            mansion_type: "Mansion".to_string(),
        },
    ];

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

    let selected_mansions = match conn.query_map(
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
    ) {
        Ok(it) => it,
        Err(err) => return Err(Box::new(err)),
    };

    assert_eq!(dbg!(mansions), dbg!(selected_mansions));
    println!("Yay!");

    Ok(())
}
