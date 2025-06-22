use std::result;
use crate::utils::iter::NoneIterator;

pub type Error = &'static str;
pub type Result<T> = result::Result<T, Error>;

pub type Watt = u32;
pub type Currency = u32;

pub trait Equipment {
    fn name(&self) -> &str;
    fn power(&self) -> Watt;
    fn net_price(&self) -> Currency;
    fn discount_price(&self) -> Currency;

    fn add(&mut self, #[allow(unused)] part: Box<dyn Equipment>) -> Result<()> {
        Err("function `Equipment::add` cannot be applied to a leaf equipment")
    }

    fn remove(&mut self, #[allow(unused)] part_name: &str) -> Result<()> {
        Err("function `Equipment::remove` cannot be applied to a leaf equipment")
    }

    fn parts(&self) -> Box<dyn Iterator<Item = &Box<dyn Equipment + '_>> + '_> {
        Box::new(NoneIterator::new())
    }
}

pub struct FloppyDisk {
    name: String,
    power: Watt,
    net_price: Currency,
    discount_price: Currency,
}

impl FloppyDisk {
    pub fn new(name: &str, power: Watt, net_price: Currency, discount_price: Currency) -> Self {
        FloppyDisk {
            name: name.to_string(),
            power,
            net_price,
            discount_price,
        }
    }
}

impl Equipment for FloppyDisk {
    fn name(&self) -> &str {
        &self.name
    }

    fn power(&self) -> Watt {
        self.power
    }

    fn net_price(&self) -> Currency {
        self.net_price
    }

    fn discount_price(&self) -> Currency {
        self.discount_price
    }
}

pub struct Chassis {
    name: String,
    power: Watt,
    net_price: Currency,
    discount_price: Currency,
    parts: Vec<Box<dyn Equipment>>,
}

impl Chassis {
    pub fn new(name: &str, power: Watt, net_price: Currency, discount_price: Currency) -> Self {
        Chassis {
            name: name.to_string(),
            power,
            net_price,
            discount_price,
            parts: Vec::new(),
        }
    }
}

impl Equipment for Chassis {
    fn name(&self) -> &str {
        &self.name
    }

    fn power(&self) -> Watt {
        let sum = self.parts.iter().fold(0, |acc, part| acc + part.power());
        sum + self.power
    }

    fn net_price(&self) -> Currency {
        let sum = self.parts.iter().fold(0, |acc, part| acc + part.net_price());
        sum + self.net_price
    }

    fn discount_price(&self) -> Currency {
        let sum = self.parts.iter().fold(0, |acc, part| acc + part.discount_price());
        sum + self.discount_price
    }

    fn add(&mut self, part: Box<dyn Equipment>) -> Result<()> {
        self.parts.push(part);
        Ok(())
    }

    fn remove(&mut self, part_name: &str) -> Result<()> {
        let pos = self.parts.iter().position(|part| part.name() == part_name);
        match pos {
            Some(pos) => {
                self.parts.remove(pos);
                Ok(())
            },
            None => Err("part not found for name"),
        }
    }

    fn parts(&self) -> Box<dyn Iterator<Item = &Box<dyn Equipment + '_>> + '_> {
        Box::new(self.parts.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composite_exam() {
        let mut chassis1 = Chassis::new("Chassis 1", 10, 100, 90);
        chassis1.add(Box::new(FloppyDisk::new("Floppy 1", 1, 10, 9))).unwrap();

        let mut chassis2 = Chassis::new("Chassis 2", 20, 200, 190);
        chassis2.add(Box::new(FloppyDisk::new("Floppy 2", 2, 20, 19))).unwrap();
        chassis2.add(Box::new(FloppyDisk::new("Floppy 3", 3, 30, 29))).unwrap();
        chassis2.add(Box::new(chassis1)).unwrap();
        assert_eq!(chassis2.parts().count(), 3);
        assert_eq!(chassis2.net_price(), 360);

        chassis2.remove("Floppy 3").unwrap();
        assert_eq!(chassis2.parts().count(), 2);
        assert_eq!(chassis2.net_price(), 330);
    }
}