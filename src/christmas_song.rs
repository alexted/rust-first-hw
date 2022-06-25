pub mod christmas_song {

    fn get_day_by_number(number: usize) -> &'static str {
        let days = [
                "first",
                "second",
                "third",
                "forth",
                "fifth",
                "sixth",
                "seventh",
                "eighth",
                "ninth",
                "tenth",
                "eleventh",
                "twelfth",
        ];
        days[number]
    }

    fn calculate_gifts(day_number: usize) -> String {
        let mut daily_gifts = [
            "And a partridge in a pear tree\n",
            "Two turtle doves and\n",
            "Three French hens\n",
            "Four calling birds\n",
            "Five golden rings\n",
            "Six geese a-laying\n",
            "Seven swans a-swimming\n",
            "Eight maids a-milking\n",
            "Nine ladies dancing\n",
            "Ten lords a-leaping\n",
            "Eleven pipers piping\n",
            "Twelve drummers drumming\n",
        ];

        if day_number == 0 {
            daily_gifts[0] = "A partridge in a pear tree\n";
        }

        let mut gifts = String::new();
        for i in (0..=day_number).rev() {
            gifts.push_str(daily_gifts[i]);
        }
        gifts
    }


    pub fn sing_song() {
        println!("Twelve Days of Christmas\n");

        for i in 0..12 {
            let day = get_day_by_number(i);
            println!("On the {day} day of Christmas,\nMy true love sent to me");
            let gifts = calculate_gifts(i);
            println!("{gifts}");
        }
    }
}