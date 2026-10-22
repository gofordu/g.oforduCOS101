use std::io::Write;

fn main() {
    let names = ["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministries = ["Internal Affairs","Justice","Defense", "Power & Steel", "Petroleum"];
    let zones = ["South West","North East","South South","South West","South East"];

    let mut file = std::fs::File::create("convicted_ministers.txt").expect("Failed to create file");

    file.write_all(b"Names           | Ministry    | Zone        \n").expect("Write failed");
    file.write_all(b"--------------------------------------------\n").expect("Write failed");

    for i in 0..5 {
        file.write_all(names[i].as_bytes()).expect("Write failed");
        file.write_all(b" | ").expect("Write failed");
        file.write_all(ministries[i].as_bytes()).expect("Write failed");
        file.write_all(b" | ").expect("Write failed");
        file.write_all(zones[i].as_bytes()).expect("Write failed");
        file.write_all(b"\n").expect("Write failed");
    }

    println!("Minister records saved to 'convicted_ministers.txt'.");
}