struct ImportantText<'a> {
    text: &'a str,
}

struct TextAnalysis<'a> {
    text: & 'a str,
    longest_word: & 'a str
}

fn analyze(text: &str) -> TextAnalysis {
    let mut longest = ""; // longest will only point to slice of borrowed string literal in for loop
    for token in text.split_whitespace() {
        if token.len() > longest.len() {
            longest = token;
        }
    }

    TextAnalysis {
        text,
        longest_word: longest,
    }
}

fn main() {
    let important_text = ImportantText {
        text: "My name is Ivan, and i am the most goated Coder and cook in the whole wide world"
    };

    let result = analyze(important_text.text);

    // result is borrowing the longest word directly from important_text
    println!("Text: {}", result.text);
    println!("Longest word: {}", result.longest_word);
}
