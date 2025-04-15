/// Prints the lyrics to "The Twelve Days of Christmas" in the specified format
pub fn twelve_days_of_christmas() {
    let gifts = [
        "a partridge in a pear tree.", // Lowercase for use with "And"
        "Two turtle doves",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for day in 1..=12 {
        println!("\nOn the {} day of Christmas", ordinal(day));
        println!("My true love gave to me");

        // Special case for first day
        if day == 1 {
            println!("A partridge in a pear tree."); // Capital A when standalone
            continue;
        }

        // Print gifts in reverse order for current day
        for gift_num in (1..day).rev() {
            println!("{}", gifts[gift_num]);
        }
        println!("And {}", gifts[0]); // Lowercase after "And"
    }
}

/// Helper function to get ordinal number (1st, 2nd, etc.)
fn ordinal(n: usize) -> String {
    match n {
        1 => "first".to_string(),
        2 => "second".to_string(),
        3 => "third".to_string(),
        4 => "fourth".to_string(),
        5 => "fifth".to_string(),
        6 => "sixth".to_string(),
        7 => "seventh".to_string(),
        8 => "eighth".to_string(),
        9 => "ninth".to_string(),
        10 => "tenth".to_string(),
        11 => "eleventh".to_string(),
        12 => "twelfth".to_string(),
        _ => format!("{}th", n),
    }
}
