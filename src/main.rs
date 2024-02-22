// Author:      Nicholas Roberts
// Date:        2/20/24
// Description: Prints out the 12 days of christmas song.

fn main() {
    let mut twelve_days: [String; 12] = Default::default();

    let mut lyric = String::new();
    for i in 0..12 {
        println!("On the {} day of Christmas,", convert_day_to_word(&i));
        println!("my true love gave to me");

        lyric = find_day_of_christmas_lyric(&i);
        twelve_days[i as usize] = lyric;

        for j in (0..i + 1).rev() {
            if i != 0 && j == 0 {
                print!("and ")
            };
            println!("{}", twelve_days[j as usize]);
        }

        println!("");
    }
}

fn find_day_of_christmas_lyric(day: &u8) -> String {
    let day_of_christmas = match day {
        0 => "A partridge in a pear tree!",
        1 => "Two turtle doves,",
        2 => "Three french hens,",
        3 => "Four calling birds,",
        4 => "FIVE GOLDEN RINGS!",
        5 => "Six geese a-laying",
        6 => "Seven swans a-swimming,",
        7 => "Eight maids a-milking,",
        8 => "Nine ladies dancing,",
        9 => "Ten lords a-leaping,",
        10 => "Eleven pipers piping,",
        11 => "Twelve drummers drumming,",
        _ => "No match",
    };

    day_of_christmas.to_string()
}

fn convert_day_to_word(day: &u8) -> String {
    let day_word = match day {
        0 => "first",
        1 => "second",
        2 => "third",
        3 => "fourth",
        4 => "fifth",
        5 => "sixth",
        6 => "seventh",
        7 => "eighth",
        8 => "nineth",
        9 => "tenth",
        10 => "eleventh",
        11 => "twelfth",
        _ => "No match",
    };

    day_word.to_string()
}
