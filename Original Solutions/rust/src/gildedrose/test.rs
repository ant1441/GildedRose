use super::{Item, GildedRose};

#[test]
pub fn test_item_name_doesnt_change() {
    let items = vec![Item::new(String::from("foo"), 0, 0)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!("foo", rose.items[0].name);
}
