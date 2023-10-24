#[allow(dead_code)]
#[derive(Eq, PartialEq, Debug)]
enum PizzaStatus {
    Ordered,
    Cooking,
    Cooked,
    Delivering,
    Delivered,
}

#[allow(dead_code)]
impl PizzaStatus {
    fn from_delivery_time(ordered_days_ago: u32) -> Self {
        match ordered_days_ago {
            0..=1 => PizzaStatus::Ordered,
            2..=6 => PizzaStatus::Cooking,
            7..=9 => PizzaStatus::Cooked,
            10..=16 => PizzaStatus::Delivering,
            _ => PizzaStatus::Delivered,
        }
    }

    fn get_delivery_time_in_days(&self) -> u32 {
        match self {
            PizzaStatus::Ordered => 17,
            PizzaStatus::Cooking => 15,
            PizzaStatus::Cooked => 10,
            PizzaStatus::Delivering => 7,
            PizzaStatus::Delivered => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::PizzaStatus;

    #[test]
    fn test_from_delivery_time() {
        assert_eq!(PizzaStatus::from_delivery_time(0), PizzaStatus::Ordered);
        assert_eq!(PizzaStatus::from_delivery_time(4), PizzaStatus::Cooking);
        assert_eq!(PizzaStatus::from_delivery_time(8), PizzaStatus::Cooked);
        assert_eq!(PizzaStatus::from_delivery_time(11), PizzaStatus::Delivering);
        assert_eq!(PizzaStatus::from_delivery_time(45), PizzaStatus::Delivered);
    }

    #[test]
    fn test_get_delivery_time() {
        let tmp = PizzaStatus::Ordered;
        assert_eq!(tmp.get_delivery_time_in_days(), 17);
        let tmp = PizzaStatus::Cooking;
        assert_eq!(tmp.get_delivery_time_in_days(), 15);
        let tmp = PizzaStatus::Cooked;
        assert_eq!(tmp.get_delivery_time_in_days(), 10);
        let tmp = PizzaStatus::Delivering;
        assert_eq!(tmp.get_delivery_time_in_days(), 7);
        let tmp = PizzaStatus::Delivered;
        assert_eq!(tmp.get_delivery_time_in_days(), 0);
    }
}
