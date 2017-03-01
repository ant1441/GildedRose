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
            GildedRose::handle_quality(&mut item);
            GildedRose::decrease_sell_by(&mut item);
        }
    }

    fn handle_quality(item: &mut Item) {
        match (item.name.as_ref(), item.quality)  {
            ("Sulfuras, Hand of Ragnaros", _) => return,
            (_, n) if n >= 50 => return,
            (_, _) => (),
        }

        if item.name == "Backstage passes to a TAFKAL80ETC concert" {
            item.quality += 1;
            if item.sell_in < 11 {
                if item.sell_in < 6 {
                    item.quality += 1;
                }
                item.quality += 1;
            }
            if item.sell_in <= 0 {
                item.quality = 0;
            }
            return;
        }
        if item.name == "Aged Brie" {
            item.quality += 1;
            if item.sell_in <= 0 {
                item.quality += 1;
            }
            return;
        }
        if item.quality > 0 {
            item.quality -= 1;
            if item.name.starts_with("Conjured") {
                item.quality -= 1;
            }
        }

        if item.sell_in <= 0 {
            if item.quality > 0 {
                item.quality -= 1;
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
