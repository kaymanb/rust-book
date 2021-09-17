pub fn add_two(i: i32) -> i32 {
    i + 2
}


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn assert_macro() {
        assert!(true);
    }
    
    #[test]
    #[should_panic(expected = "Oops")]
    fn please_panic() {
        panic!("Oops");
    }
    
    #[test]
    fn result() -> Result<(), String> {
        Ok(()) 
        //Err(String::from("Error!"))
    }

}
