fn main() {
    // Using a vector of tuples to store APS levels and their corresponding descriptions
    let aps_levels: Vec<(&str, &str)> = vec![
        ("APS 1-2", "Intern / Placement / Paralegal"),
        ("APS 3-5", "Administrator / Junior Associate / Classroom Teacher"),
        ("APS 5-8", "Senior Administrator / Associate / Senior Teacher"),
        ("EL1 8-10", "Office Manager / Senior Associate 1-2 / Leading Teacher"),
        ("EL2 10-13", "Director / Senior Associate 3-4 / Deputy Principal"),
        ("SES", "CEO / Dean / Partner / Principal"),
    ];

    // Simulating a staff query based on experience bracket (e.g., index 2 maps to APS 5-8)
    let target_experience_bracket = 2; 

    println!("--- Public Service APS Level Checker ---");
    
    // Accessing the vector safely using indexing
    if let Some((level, roles)) = aps_levels.get(target_experience_bracket) {
        println!("Grade Level: {}", level);
        println!("Eligible Roles: {}", roles);
    } else {
        println!("Invalid level bracket entered.");
    }
}