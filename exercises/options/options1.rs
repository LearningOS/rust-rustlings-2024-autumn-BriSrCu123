// options1.rs
//
// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
    fn maybe_icecream(time_of_day: u16) -> Option<u16> {
        if time_of_day < 22 {
            Some(5)
        } else if time_of_day == 22 {
            Some(0)
        } else {
            None
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn check_icecream() {
            assert_eq!(maybe_icecream(9), Some(5));
            assert_eq!(maybe_icecream(10), Some(5));
            assert_eq!(maybe_icecream(22), Some(0));
            assert_eq!(maybe_icecream(23), None);
            assert_eq!(maybe_icecream(25), None);
        }
    
        #[test]
        fn raw_value() {
            match maybe_icecream(12) {
                Some(icecreams) => assert_eq!(icecreams, 5),
                None => panic!("Expected Some value but got None"),
            }
        }
    }
    
    