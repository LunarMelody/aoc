use super::input::INPUT_DATA;

pub fn part1() {
    let elfs = INPUT_DATA.split("\n\n");
    let most_calories = elfs
        .map(|inv| {
            inv.lines()
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!("Elf carrying {} calories has the most calories!", most_calories);
}

pub fn part2() {

}
