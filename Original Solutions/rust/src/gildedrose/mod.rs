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
        if item.name == "Sulfuras, Hand of Ragnaros" {
            return;
        }
        if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert" {
            if item.quality > 0 {
                item.quality = item.quality - 1;
                if item.name.starts_with("Conjured") {
                    item.quality = item.quality - 1;
                }
            }
        } else {
            if item.quality < 50 {
                item.quality = item.quality + 1;

                if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                    if item.sell_in < 11 && item.quality < 50 {
                        if item.sell_in < 6 {
                            item.quality = item.quality + 1;
                        }
                        item.quality = item.quality + 1;
                    }

                }
            }
        }

        if item.sell_in <= 0 {
            if item.name == "Aged Brie" {
                if item.quality < 50 {
                    item.quality = item.quality + 1;
                }
            } else if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                item.quality = 0;
            } else if item.quality > 0 {
                item.quality = item.quality - 1;
            }
        }

    }

    fn decrease_sell_by(item: &mut Item) {
        if item.name != "Sulfuras, Hand of Ragnaros" {
            item.sell_in = item.sell_in - 1;
        }
    }
}

#[cfg(test)]
mod test;
