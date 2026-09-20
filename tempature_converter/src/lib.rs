fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0/5.0) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_to_c0() {
        let result = fahrenheit_to_celsius(32.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn f212_to_c100() {
        let result = fahrenheit_to_celsius(212.0);
        let expected = 100.0;
        assert!((result-expected).abs() < 0.001);
    }

    #[test]
    fn c0_to_f32(){
        let result = celsius_to_fahrenheit(0.0);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn c100_to_f212(){
        let result = celsius_to_fahrenheit(100.0);
        let expected = 212.0;
        assert!((result - expected).abs() < 0.001);
    }
}
