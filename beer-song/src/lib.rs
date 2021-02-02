#![allow(unused)]

pub fn verse(n: u32) -> String {
    match n {
        y if y == 0 => { String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n") },
        y  => {
            String::from(format!("{n1} of beer on the wall, {n1} of beer.\nTake {count} down and pass it around, {n2} of beer on the wall.\n", n1={
                if y == 1 {
                    "1 bottle".to_owned()
                } else {
                    format!("{} bottles", y).to_owned()
                }
            }, n2={
                let next = y - 1;
                if next == 0 {
                    "no more bottles".to_owned()
                } else if next == 1 {
                    "1 bottle".to_owned()
                } else {
                    format!("{} bottles", next).to_owned()
                }
            }, count=if y == 1 {
                "it".to_owned()
            } else {
                "one".to_owned()
            }))
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {

    (end..=start).rev()
                 .map(|x| verse(x))
                 .collect::<Vec<_>>()
                 .join("\n")


    // let mut s = String::new();

    // for i in (end..=start).rev() {
    //     s.push_str(&verse(i));
    //     if i != end {
    //         s.push('\n')
    //     }
    // }

    // s
}
