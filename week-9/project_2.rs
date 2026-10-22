use std::io::Write;

fn main() {
    let names = ["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matrics = ["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let depts = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let levels = ["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("pau_smis_records.txt").expect("Failed to create file");

    file.write_all(b"Student Name         | Matric. Number  | Department   | Level\n").expect("Write failed");
    file.write_all(b"-----------------------------------------------------------\n").expect("Write failed");

    for i in 0..5 {
        file.write_all(names[i].as_bytes()).expect("Write failed");
        file.write_all(b" | ").expect("Write failed");
        file.write_all(matrics[i].as_bytes()).expect("Write failed");
        file.write_all(b" | ").expect("Write failed");
        file.write_all(depts[i].as_bytes()).expect("Write failed");
        file.write_all(b" | ").expect("Write failed");
        file.write_all(levels[i].as_bytes()).expect("Write failed");
        file.write_all(b"\n").expect("Write failed");
    }

    println!("Student records saved to 'pau_smis_records.txt'.");
}