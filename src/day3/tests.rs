#[test]
fn english_alphabet_with_positions() {
    let scores_map = super::solution::get_char_priorities();

    assert_eq!(scores_map.get(&'a').unwrap(), &1);
    assert_eq!(scores_map.get(&'z').unwrap(), &26);
    assert_eq!(scores_map.get(&'A').unwrap(), &27);
    assert_eq!(scores_map.get(&'Z').unwrap(), &52);
}


#[test]
fn are_rucksack_items_even() {
    super::input::INPUT_DATA.lines().for_each(|rucksack| {
        let remainder = rucksack.len().wrapping_rem(2);
        assert_eq!(remainder, 0);
    });
}
