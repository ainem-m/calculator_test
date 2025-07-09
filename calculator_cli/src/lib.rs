pub fn calculate(value1: f64, operator: &str, value2: f64) -> Result<f64, String> {
    match operator {
        "+" => Ok(value1 + value2),
        "-" => Ok(value1 - value2),
        "*" => Ok(value1 * value2),
        "/" => {
            if value2 == 0.0 {
                Err(String::from("Error: Division by zero"))
            } else {
                Ok(value1 / value2)
            }
        }
        _ => Err(format!("Error: Invalid operator '{}'", operator)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(calculate(2.0, "+", 2.0), Ok(4.0));
    }

    #[test]
    fn test_subtract() {
        assert_eq!(calculate(4.0, "-", 2.0), Ok(2.0));
    }

    #[test]
    fn test_multiply() {
        assert_eq!(calculate(3.0, "*", 3.0), Ok(9.0));
    }

    #[test]
    fn test_divide() {
        assert_eq!(calculate(10.0, "/", 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(
            calculate(10.0, "/", 0.0),
            Err(String::from("Error: Division by zero"))
        );
    }

    #[test]
    fn test_invalid_operator() {
        assert_eq!(
            calculate(10.0, "%", 2.0),
            Err(String::from("Error: Invalid operator '%'"))
        );
    }

    #[test]
    fn test_decimal_add() {
        assert_eq!(calculate(1.2, "+", 3.4), Ok(4.6));
    }

    #[test]
    fn test_decimal_division() {
        assert_eq!(calculate(5.5, "/", 2.0), Ok(2.75));
    }

    #[test]
    fn test_calculation_overflow() {
        let large_number = 1.0e308;
        assert_eq!(calculate(large_number, "*", 2.0), Ok(f64::INFINITY));
    }
}
