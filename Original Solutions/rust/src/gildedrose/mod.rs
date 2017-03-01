use std::string;
use std::vec;

pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32,
}

#[derive(PartialEq, Eq)]
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

    fn is_conjured(&self) -> bool {
        self.name.starts_with("Conjured")
    }

    fn increment_quality(&self, n: i32) -> i32 {
        if self.is_conjured() {
            self.quality + (2 * n)
        } else {
            self.quality + n
        }
    }

    fn decrement_quality(&self, n: i32) -> i32 {
        if self.is_conjured() {
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
        match (item.type_of(), item.quality, item.sell_in) {
            (Type::Legendary, quality, _) => quality,

            (_, quality, _) if quality >= 50 => quality,

            (Type::RareWithTime, _, sell_in) if sell_in <= 0 => 0,
            (Type::RareWithTime, _, 1...5) => item.increment_quality(3),
            (Type::RareWithTime, _, 6...10) => item.increment_quality(2),
            (Type::RareWithTime, _, _) => item.increment_quality(1),

            (Type::Rare, _, sell_in) if sell_in <= 0 => item.increment_quality(2),
            (Type::Rare, _, _) => item.increment_quality(1),

            (Type::Normal, quality, _) if quality <= 0 => quality,
            (Type::Normal, _, sell_in) if sell_in <= 0 => item.decrement_quality(2),
            (Type::Normal, _, _) => item.decrement_quality(1),
        }
    }

    fn decrease_sell_by(item: &Item) -> i32 {
        if item.type_of() == Type::Legendary {
            item.sell_in
        } else {
            item.sell_in - 1
        }
    }
}

#[cfg(test)]
mod test;
