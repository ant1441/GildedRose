use super::{Item, GildedRose};

#[test]
pub fn test_item_name_doesnt_change() {
    let items = vec![Item::new(String::from("foo"), 0, 0)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!("foo", rose.items[0].name);
}

#[test]
pub fn test_sell_in_decreases_by_one_each_update() {
    let items = vec![Item::new(String::from("foo"), 10, 10)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(9, rose.items[0].sell_in);
}

#[test]
pub fn test_quality_decreases_by_one_each_update() {
    let items = vec![Item::new(String::from("foo"), 10, 10)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(9, rose.items[0].quality);
}

#[test]
pub fn test_quality_decreases_by_double_each_update_after_sell_by() {
    let items = vec![Item::new(String::from("foo"), 1, 10)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();
    assert_eq!(9, rose.items[0].quality);

    rose.update_quality();
    assert_eq!(7, rose.items[0].quality);
}

#[test]
pub fn test_quality_never_goes_below_zero() {
    let items = vec![Item::new(String::from("foo"), 1, 1)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();
    assert_eq!(0, rose.items[0].quality);

    rose.update_quality();
    assert_eq!(0, rose.items[0].quality);

    rose.update_quality();
    assert_eq!(0, rose.items[0].quality);
}

#[test]
pub fn test_aged_brie_increases_in_quality() {
    let items = vec![Item::new(String::from("Aged Brie"), 10, 45)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();
    assert_eq!(46, rose.items[0].quality);

    for _ in 0..3 {
        rose.update_quality();
    }
    assert_eq!(49, rose.items[0].quality);

    rose.update_quality();
    assert_eq!(50, rose.items[0].quality);
    rose.update_quality();
    assert_eq!(50, rose.items[0].quality);
    rose.update_quality();
    assert_eq!(50, rose.items[0].quality);
}

#[test]
pub fn test_sulfuras_never_changes() {
    let items = vec![Item::new(String::from("Sulfuras, Hand of Ragnaros"), 10, 80)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(80, rose.items[0].quality);
    assert_eq!(10, rose.items[0].sell_in);

    rose.update_quality();

    assert_eq!(80, rose.items[0].quality);
    assert_eq!(10, rose.items[0].sell_in);
}

#[test]
pub fn test_backstage_passes_increases_in_quality() {
    let items = vec![Item::new(String::from("Backstage passes to a TAFKAL80ETC concert"),
                               20,
                               45)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();
    assert_eq!(46, rose.items[0].quality);

    for _ in 0..3 {
        rose.update_quality();
    }
    assert_eq!(49, rose.items[0].quality);

    rose.update_quality();
    assert_eq!(50, rose.items[0].quality);
    rose.update_quality();
    assert_eq!(50, rose.items[0].quality);
    rose.update_quality();
    assert_eq!(50, rose.items[0].quality);
}

#[test]
pub fn test_backstage_passes_increases_in_quality_faster_when_its_closer_to_sell_by() {
    let items = vec![Item::new(String::from("Backstage passes to a TAFKAL80ETC concert"),
                               11,
                               20)];
    let mut rose = GildedRose::new(items);

    rose.update_quality();
    assert_eq!(21, rose.items[0].quality);

    rose.update_quality(); // 9 days
    assert_eq!(23, rose.items[0].quality);
    rose.update_quality(); // 8 days
    assert_eq!(25, rose.items[0].quality);
    rose.update_quality(); // 7 days
    assert_eq!(27, rose.items[0].quality);

    rose.update_quality(); // 6 days
    rose.update_quality(); // 5 days

    rose.update_quality(); // 4 days
    assert_eq!(34, rose.items[0].quality);
    rose.update_quality(); // 3 days
    assert_eq!(37, rose.items[0].quality);

    rose.update_quality(); // 2 days
    rose.update_quality(); // 1 days

    rose.update_quality(); // 0 days
    assert_eq!(46, rose.items[0].quality);
    rose.update_quality(); // -1 days
    assert_eq!(0, rose.items[0].quality);
}

#[test]
pub fn test_conjured_items_decrease_in_quality_twice_as_fast() {
    let items = vec![Item::new(String::from("Conjured Mana Cake"), 10, 10)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!(8, rose.items[0].quality);
}
