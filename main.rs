/*fn unique_words(sentence: &str) -> Vec<String> {
    // Split the sentence into words
    let words: Vec<&str> = sentence.split_whitespace().collect();

    let unique_words: std::collections::HashSet<String> = words
        .into_iter()
        .map(|word| word.to_lowercase())
        .collect();

    // Convert the HashSet back to a sorted vector and return it
    let mut sorted_unique_words: Vec<String> = unique_words.into_iter().collect();
    sorted_unique_words.sort(); // Sort the words alphabetically
    sorted_unique_words
}

fn main() {
    let sentence = "This is a sample sentence with some words. This is a sample.";

    // Call the unique_words function and print the result
    let unique_words = unique_words(sentence);
    println!("Unique Words: {:?}", unique_words);
}
// question2
fn main() {
    // Example input vector of integers
    let input_numbers = vec![1, 2, 3, 4, 5];

    // Square each element and calculate the sum of squared values
    let sum_of_squares: i32 = input_numbers
        .iter()
        .map(|&x| x * x) // Square each element
        .sum(); // Calculate the sum

    // Print the result
    println!("Sum of squared values: {}", sum_of_squares);
}*
// question3
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    // Open the text file
    let file = File::open("example.txt").expect("Failed to open file");

    // Create a regular expression to match words (ignoring punctuation)
    let word_regex = Regex::new(r"[a-zA-Z]+").expect("Invalid regex pattern");

    // Create a HashMap to store word frequencies
    let mut word_counts: HashMap<String, u32> = HashMap::new();

    // Read the file line by line and count word frequencies
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            for word in word_regex.find_iter(&line) {
                let word = word.as_str().to_lowercase();
                let count = word_counts.entry(word).or_insert(0);
                *count += 1;
            }
        }
    }

    // Print word frequencies
    for (word, count) in &word_counts {
        println!("Word: {}, Frequency: {}", word, count);
    }
}*
//question 4
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    // Attempt to open the file
    let mut file = File::open(path)?;

    // Read the file content into a String
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn main() {
    let file_path = "example.txt"; 
    match read_file(file_path) {
        Ok(content) => {
            println!("File content:\n{}", content);
        }
        Err(error) => {
            println!("Error reading the file: {}", error);
        }
    }
}*
// question 5
fn parse_integer(input: &str) -> Option<i32> {
    match input.parse::<i32>() {
        Ok(parsed_integer) => Some(parsed_integer),
        Err(_) => None,
    }
}

fn main() {
    let input_string = "42"; // Replace this with the string you want to parse

    match parse_integer(input_string) {
        Some(parsed_integer) => {
            println!("Parsed Integer: {}", parsed_integer);
        }
        None => {
            println!("Failed to parse the integer.");
        }
    }
}*
// question 6
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// Define a custom error type
#[derive(Debug)]
enum ConfigError {
    InvalidFormat,
    IoError(io::Error),
}

// Implement the From trait to convert IO errors into ConfigError
impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        ConfigError::IoError(error)
    }
}

// Function to read and process the configuration file
fn read_config_file(file_path: &str) -> Result<String, ConfigError> {
    let mut content = String::new();

    // Open and read the file
    let mut file = File::open(file_path)?;

    // Read file content into a String
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn main() {
    let file_path = "example.txt";

    match read_config_file(file_path) {
        Ok(config_content) => {
            println!("Configuration file content: {}", config_content);
        }
        Err(error) => {
            match error {
                ConfigError::InvalidFormat => println!("Invalid configuration file format."),
                ConfigError::IoError(io_error) => println!("IO Error: {}", io_error),
            }
        }
    }
}
// question 7
fn filter_elements<T, F>(input: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    input.into_iter().filter(|&x| predicate(&x)).collect()
}

fn main() {
    // Example usage for filtering integers
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filtered_numbers = filter_elements(numbers, |&x| x % 2 == 0);
    println!("Filtered Numbers: {:?}", filtered_numbers);

    // Example usage for filtering strings
    let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    let filtered_words = filter_elements(words, |&x| x.len() > 5);
    println!("Filtered Words: {:?}", filtered_words);
}
// question 8
// Defining the trait Drawable
trait Drawable {
    fn draw(&self);
}

// Creating a struct Shape
struct Shape {
    name: String,
}

// Implementing the Drawable trait for Shape
impl Drawable for Shape {
    // Implementing the draw method for Shape
    fn draw(&self) {
        println!("Drawing shape: {}", self.name);
    }
}

fn main() {
    // Creating a Shape instance
    let rectangle = Shape { name: String::from("Rectangle") };
    
    // Calling the draw method on the Shape instance
    rectangle.draw(); // Output: "Drawing shape: Rectangle"
}
// question 9
// Define a trait for calculating area
trait Area {
    fn calculate_area(&self) -> f64;
}

// Define a trait for calculating perimeter
trait Perimeter {
    fn calculate_perimeter(&self) -> f64;
}

// Define a struct for a circle
struct Circle {
    radius: f64,
}

// Implement the Area trait for Circle
impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Implement the Perimeter trait for Circle
impl Perimeter for Circle {
    fn calculate_perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// Define a struct for a rectangle
struct Rectangle {
    width: f64,
    height: f64,
}

// Implement the Area trait for Rectangle
impl Area for Rectangle {
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }
}

// Implement the Perimeter trait for Rectangle
impl Perimeter for Rectangle {
    fn calculate_perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    // Example usage of the geometry library
    let circle = Circle { radius: 5.0 };
    println!("Area of the circle: {}", circle.calculate_area());
    println!("Perimeter of the circle: {}", circle.calculate_perimeter());

    let rectangle = Rectangle { width: 3.0, height: 4.0 };
    println!("Area of the rectangle: {}", rectangle.calculate_area());
    println!("Perimeter of the rectangle: {}", rectangle.calculate_perimeter());
}*/
// questiuon 11
use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Get user input
    println!("Enter text to append to the file:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Specify the file path
    let file_path = "example.txt";

    // Open the file in append mode or create it if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    // Write the user input to the file
    file.write_all(input.as_bytes())?;

    println!("Text appended to the file successfully!");


    Ok(())
}

