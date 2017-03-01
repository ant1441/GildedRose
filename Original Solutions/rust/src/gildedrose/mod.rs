use std::string;
use std::vec;

pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32,
}

enum Type {
    Legendary,
    Rare,
    RareWithTime,
    Normal,
}

impl Item {
    pub fn new(name: String, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name,
            sell_in: sell_in,
            quality: quality,
        }
    }

    fn type_of(&self) -> Type {
        match self.name.as_ref() {
            "Sulfuras, Hand of Ragnaros" => Type::Legendary,
            "Backstage passes to a TAFKAL80ETC concert" => Type::RareWithTime,
            "Aged Brie" => Type::Rare,
            _ => Type::Normal,
        }
    }

    fn increment_quality(&self, n: i32) -> i32 {
        if self.name.starts_with("Conjured") {
            self.quality + (2 * n)
        } else {
            self.quality + n
        }
    }

    fn decrement_quality(&self, n: i32) -> i32 {
        if self.name.starts_with("Conjured") {
            self.quality - (2 * n)
        } else {
            self.quality - n
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
            item.quality = GildedRose::handle_quality(&item);
            item.sell_in = GildedRose::decrease_sell_by(&item);
        }
    }

    fn handle_quality(item: &Item) -> i32 {
        match (item.type_of(), item.quality) {
            (Type::Legendary, n) => n,
            (_, n) if n >= 50 => n,
            (Type::RareWithTime, _) => {
                match item.sell_in {
                    n if n <= 0 => 0,
                    1...5 => item.increment_quality(3),
                    6...10 => item.increment_quality(2),
                    _ => item.increment_quality(1),
                }
            }
            (Type::Rare, _) => {
                match item.sell_in {
                    n if n <= 0 => item.increment_quality(2),
                    _ => item.increment_quality(1),
                }
            }
            (Type::Normal, n) if n <= 0 => item.quality,
            (Type::Normal, _) => {
                if item.sell_in <= 0 {
                    return item.decrement_quality(2);
                } else {
                    return item.decrement_quality(1);
                }
            }
        }
    }

    fn decrease_sell_by(item: &Item) -> i32 {
        if item.name == "Sulfuras, Hand of Ragnaros" {
            item.sell_in
        } else {
            item.sell_in - 1
        }
    }
}

#[cfg(test)]
mod test;
