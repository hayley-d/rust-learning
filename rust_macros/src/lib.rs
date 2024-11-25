pub mod macros {
    #[macro_export]
    macro_rules! vec {
        // look at it like a match expression
        // $($x:expr),* is the pattern being matched
        // vec![1,2,3]
        ($( $x:expr),*) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
}

//* the means 0 or more times the pattern can be matched
// $x is a placeholder for a rust expression
