#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)] //usef for test purpose
enum PizzaStatus {
    Ordered,
    Cooking,
    Cooked,
    Delivering,
    Delivered,
}

#[allow(dead_code)]
impl PizzaStatus {
    fn from_delivery_time( ordered_days_ago:u32 ) -> Self {
        if ordered_days_ago <= 2 {
            return Self::Ordered;
        }
        else if ordered_days_ago <= 7 {
            return Self::Cooking;
        }
        else if ordered_days_ago <= 10 {
            return  Self::Cooked;
        }
        else if ordered_days_ago <= 17 {
            return Self::Delivering;
        }
        Self::Delivered
    }

    fn get_delivery_time( &self ) ->u32 {
       match self {
            PizzaStatus::Ordered =>  16,
            PizzaStatus::Cooking =>  14,
            PizzaStatus::Cooked =>  9,
            PizzaStatus::Delivering =>  6,
            PizzaStatus::Delivered =>  0,
        }     
    }
}

#[cfg(test)]
mod test {
    use crate::PizzaStatus;
    
    #[test]
    fn test_get_delivery_time() {
        let test: PizzaStatus = PizzaStatus::Ordered;
        assert_eq!(test.get_delivery_time(), 16);
        let test: PizzaStatus = PizzaStatus::Cooking;
        assert_eq!(test.get_delivery_time(), 14);
        let test: PizzaStatus = PizzaStatus::Cooked;
        assert_eq!(test.get_delivery_time(), 9);
        let test: PizzaStatus = PizzaStatus::Delivering;
        assert_eq!(test.get_delivery_time(), 6);
        let test: PizzaStatus = PizzaStatus::Delivered;
        assert_eq!(test.get_delivery_time(), 0);
    }

    #[test]
    fn test_from_time() {
        assert_eq!(PizzaStatus::from_delivery_time(0), PizzaStatus::Ordered);
        assert_eq!(PizzaStatus::from_delivery_time(1), PizzaStatus::Ordered);
        assert_eq!(PizzaStatus::from_delivery_time(5), PizzaStatus::Cooking);
        assert_eq!(PizzaStatus::from_delivery_time(7), PizzaStatus::Cooking);
        assert_eq!(PizzaStatus::from_delivery_time(9), PizzaStatus::Cooked);
        assert_eq!(PizzaStatus::from_delivery_time(10), PizzaStatus::Cooked);
        assert_eq!(PizzaStatus::from_delivery_time(11), PizzaStatus::Delivering);
        assert_eq!(PizzaStatus::from_delivery_time(17), PizzaStatus::Delivering);
        assert_eq!(PizzaStatus::from_delivery_time(18), PizzaStatus::Delivered);
        assert_eq!(PizzaStatus::from_delivery_time(64564), PizzaStatus::Delivered);
    }
}