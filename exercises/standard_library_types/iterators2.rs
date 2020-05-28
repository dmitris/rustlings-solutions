// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer.
// Step 1. Complete the `capitalize_first` function to pass the first two cases.
// Step 2. Apply the `capitalize_first` function to a vector of strings.
//         Ensure that it returns a vector of strings as well.
// Step 3. Apply the `capitalize_first` function again to a list.
//         Try to ensure it returns a single string.
// As always, there are hints if you execute `rustlings hint iterators2`!

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let a = c.next();
    match a {
        Some('a'..='z') => a.unwrap().to_uppercase().to_string() + &c.collect::<String>(),
        Some(_) => c.collect::<String>(),
        None => String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = words.into_iter().map(
            |s| capitalize_first(s)
        ).collect::<Vec<String>>();
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let vec = words.into_iter().map(
            |s| capitalize_first(s)
        )
        .filter(|s| s.len()>0)  // filter out empty strings
        .collect::<Vec<String>>();
        let capitalized_words = vec.join(" ");

        // let mut capitalized_words = "".to_owned();
        // let mut sep = "".to_owned();
        // for s in vec {
        //     if s.len() > 0 {
        //         capitalized_words += &sep;
        //         capitalized_words += &s;
        //         sep = " ".to_owned();
        //     }
        // }

        assert_eq!(capitalized_words, "Hello World");
    }
}
