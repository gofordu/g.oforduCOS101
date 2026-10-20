fn main() {
    // Vector of tuples storing candidate names and their years of experience: (name, years)
    let candidates: Vec<(&str, u32)> = vec![
        ("Oluchi Mordi", 6),
        ("Adams Aliyu", 10),
        ("Shania Bolade", 14),
        ("Adekunle Gold", 4),
        ("Blanca Edemoh", 9),
    ];

    // Initialize tracking variables using the first candidate
    let mut top_candidate = candidates[0].0;
    let mut max_experience = candidates[0].1;

    // Walk through the vector using a for loop to find the highest experience
    for i in 1..candidates.len() {
        let (name, years) = candidates[i];
        if years > max_experience {
            max_experience = years;
            top_candidate = name;
        }
    }

    println!("--- EY Nigeria Developer Interview ---");
    println!(
        "The candidate with the highest experience is {} with {} years.",
        top_candidate, max_experience
    );
}

//SIR I DONT QUITE UNDERSTAND HOW TO GO ABOUT THESE PROJECTS