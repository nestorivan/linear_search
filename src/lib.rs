pub fn linear_search(haystack: Vec<i8>, needle: i8) -> bool {
    for hay in haystack.iter() {
        if *hay == needle {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_find_an_existing_element() {
        let result = super::linear_search(vec![1, 2, 3, 4, 5], 3);

        assert_eq!(result, true);
    }

    #[test]
    fn should_not_find_an_element() {
        let result = super::linear_search(vec![1, 2, 3, 4, 5], 6);

        assert_eq!(result, false);
    }
}
