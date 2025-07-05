use std::collections::HashMap;

pub type Watt = u32;
pub type Currency = u32;

pub trait Equipment {
    fn name(&self) -> &str;
    fn power(&self) -> Watt;
    fn net_price(&self) -> Currency;
    fn discount_price(&self) -> Currency;
    fn accept(&mut self, visitor: &mut dyn EquipmentVisitor);
}

pub struct FloppyDisk {
    name: String,
    power: Watt,
    net_price: Currency,
    discount_price: Currency,
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

    fn accept(&mut self, visitor: &mut dyn EquipmentVisitor) {
        visitor.visit_floppy_disk(self);
    }
}

pub struct Chassis {
    name: String,
    power: Watt,
    net_price: Currency,
    discount_price: Currency,
    parts: Vec<Box<dyn Equipment>>,
}

impl Equipment for Chassis {
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

    fn accept(&mut self, visitor: &mut dyn EquipmentVisitor) {
        for part in self.parts.iter_mut() {
            part.accept(visitor);
        }
        visitor.visit_chassis(self);
    }
}

pub trait EquipmentVisitor {
    fn visit_floppy_disk(&mut self, floppy_disk: &mut FloppyDisk);
    fn visit_chassis(&mut self, chassis: &mut Chassis);
}

pub struct PricingVisitor {
    total: Currency,
}

impl PricingVisitor {
    pub fn new() -> Self {
        PricingVisitor { total: 0 }
    }

    pub fn total_price(&self) -> Currency {
        self.total
    }
}

impl EquipmentVisitor for PricingVisitor {
    fn visit_floppy_disk(&mut self, floppy_disk: &mut FloppyDisk) {
        self.total += floppy_disk.net_price();
    }

    fn visit_chassis(&mut self, chassis: &mut Chassis) {
        self.total += chassis.discount_price();
    }
}

pub type Quantity = u32;

pub struct Inventory {
    equipments: HashMap<String, Quantity>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { equipments: HashMap::new() }
    }

    pub fn accumulate(&mut self, name: &str) {
        let quantity = self.equipments.entry(name.to_string()).or_insert(0);
        *quantity += 1;
    }

    pub fn list(&self) -> &HashMap<String, Quantity> {
        &self.equipments
    }

    pub fn quantity(&self, name: &str) -> Option<&Quantity> {
        self.equipments.get(name)
    }
}

pub struct InventoryVisitor {
    inventory: Inventory,
}

impl InventoryVisitor {
    pub fn new() -> Self {
        InventoryVisitor { inventory: Inventory::new() }
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }
}

impl EquipmentVisitor for InventoryVisitor {
    fn visit_floppy_disk(&mut self, floppy_disk: &mut FloppyDisk) {
        self.inventory.accumulate(floppy_disk.name());
    }

    fn visit_chassis(&mut self, chassis: &mut Chassis) {
        self.inventory.accumulate(chassis.name());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visitor_exam1() {
        let mut equipments: Vec<Box<dyn Equipment>> = vec![
            Box::new(FloppyDisk {
                name: "Floppy".to_string(),
                power: 1,
                net_price: 10,
                discount_price: 9,
            }),
            Box::new(Chassis {
                name: "Chassis".to_string(),
                power: 10,
                net_price: 100,
                discount_price: 90,
                parts: vec![
                    Box::new(FloppyDisk {
                        name: "Floppy".to_string(),
                        power: 2,
                        net_price: 20,
                        discount_price: 19,
                    }),
                    Box::new(FloppyDisk {
                        name: "Floppy".to_string(),
                        power: 3,
                        net_price: 30,
                        discount_price: 29,
                    }),
                    Box::new(Chassis {
                        name: "Chassis".to_string(),
                        power: 20,
                        net_price: 200,
                        discount_price: 190,
                        parts: vec![
                            Box::new(FloppyDisk {
                                name: "Floppy".to_string(),
                                power: 4,
                                net_price: 40,
                                discount_price: 39,
                            }),
                        ],
                    }),
                ],
            }),
        ];

        let mut v = PricingVisitor::new();
        for e in equipments.iter_mut() {
            e.accept(&mut v);
        }
        assert_eq!(v.total_price(), 380);

        let mut v = InventoryVisitor::new();
        for e in equipments.iter_mut() {
            e.accept(&mut v);
        }
        assert_eq!(v.inventory().list(), &HashMap::from([
            ("Floppy".to_string(), 4),
            ("Chassis".to_string(), 2),
        ]));
    }
}