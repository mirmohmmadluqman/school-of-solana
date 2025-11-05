// ============================================
// STEP 1: Define the Trait (Contract)
// ============================================
// This is like a "rule" that says:
// "Any type with Summary MUST have a summarize function"

trait Summary {
    fn summarize(&self) -> String;
    // ↑
    // - fn = function
    // - summarize = function name
    // - &self = reference to the instance
    // - -> String = returns a String
    // - ; = no implementation here (just the signature)
}


// ============================================
// STEP 2: Define a Struct (Data Type)
// ============================================
// This is like a blueprint for data

struct NewsArticle {
    headline: String,
    content: String,
}

struct Tweet {
    username: String,
    message: String,
}


// ============================================
// STEP 3: Implement the Trait for NewsArticle
// ============================================
// "NewsArticle now has the Summary trait"

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // format! creates a String
        // {} gets replaced by the values
        // No semicolon = automatic return
        format!("{} - {}", self.headline, self.content)
    }
}


// ============================================
// STEP 4: Implement the Trait for Tweet
// ============================================
// "Tweet now has the Summary trait too"

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.message)
    }
}


// ============================================
// STEP 5: Generic Function Using the Trait
// ============================================
// This function works with ANY type that has Summary

fn print_summary<T: Summary>(item: &T) {
    // ↑
    // - fn print_summary = function name
    // - <T: Summary> = generic type T that MUST have Summary trait
    // - (item: &T) = parameter of type T (borrowed)
    
    println!("{}", item.summarize());
    // ↑ We can call summarize() because T has Summary trait
}


// ============================================
// STEP 6: Main Function (Using Everything)
// ============================================
fn main() {
    // Create a NewsArticle
    let article = NewsArticle {
        headline: String::from("Rust 2.0 Released"),
        content: String::from("It's faster and safer"),
    };

    // Create a Tweet
    let tweet = Tweet {
        username: String::from("rustlang"),
        message: String::from("Rust is awesome!"),
    };

    // Call summarize directly
    println!("{}", article.summarize());
    // Output: Rust 2.0 Released - It's faster and safer

    println!("{}", tweet.summarize());
    // Output: @rustlang: Rust is awesome!

    // Use the generic function
    print_summary(&article);
    // Output: Rust 2.0 Released - It's faster and safer

    print_summary(&tweet);
    // Output: @rustlang: Rust is awesome!
}