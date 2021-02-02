pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        1 => format!("And all for the want of a {}.", list[0]),
        _ => {
            let mut s = (0..list.len()-1)
                .map(move |i| format!("For want of a {} the {} was lost.", list[i], list[i+1]))
                .collect::<Vec<_>>()
                .join("\n");
            s.push_str(&format!("\nAnd all for the want of a {}.", list[0]));
            s
        }
    }
}
