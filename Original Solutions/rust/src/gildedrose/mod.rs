use std::string;
use std::vec;

pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: String, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name,
            sell_in: sell_in,
            quality: quality,
        }
    }
}

pub struct GildedRose {
    pub items: vec::Vec<Item>,
}

impl GildedRose {
    pub fn new(items: vec::Vec<Item>) -> GildedRose {
        GildedRose { items: items }
    }

    pub fn update_quality(&mut self) {
        for mut item in &mut self.items {
            item.quality = GildedRose::handle_quality(&mut item);
            GildedRose::decrease_sell_by(&mut item);
        }
    }

    fn handle_quality(item: &Item) -> i32 {
        let mut qual = item.quality;
        match (item.name.as_ref(), qual) {
            ("Sulfuras, Hand of Ragnaros", n) => n,
            (_, n) if n >= 50 => n,
            ("Backstage passes to a TAFKAL80ETC concert", _) => {
                if item.sell_in <= 0 {
                    return 0;
                }
                if item.sell_in < 6 {
                    return qual + 3;
                }
                if item.sell_in < 11 {
                    return qual + 2;
                }
                return qual + 1;
            }
            ("Aged Brie", _) => {
                if item.sell_in <= 0 {
                    return qual + 2;
                }
                return qual + 1;
            }
            (_, _) => {
                if qual <= 0 {
                    return qual;
                }
                if item.name.starts_with("Conjured") {
                    if item.sell_in <= 0 {
                        return qual - 3;
                    }
                    return qual - 2;
                }
                if item.sell_in <= 0 {
                    return qual - 2;
                }
                return qual - 1;
            }
        }
    }

    fn decrease_sell_by(item: &mut Item) {
        if item.name != "Sulfuras, Hand of Ragnaros" {
            item.sell_in -= 1;
        }
    }
}

#[cfg(test)]
mod test;
