use super::input::INPUT_DATA;

pub fn part1() {
    let elves = INPUT_DATA.split("\n\n");
    let most_calories = elves
        .map(|inv| {
            inv.lines()
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!(
        "Elf carrying {} calories has the most calories!",
        most_calories
    );
}

pub fn part2() {
    let backpacks = INPUT_DATA.split("\n\n");

    let mut calories_per_elf: Vec<u32> = backpacks
        .map(|backpack| {
            backpack
                .lines()
                .map(|entry| entry.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();

    calories_per_elf.sort_by(|a, b| b.cmp(a));

    let top_three: u32 = calories_per_elf.iter().take(3).sum();

    println!("Top three: {:?}", top_three);
}
