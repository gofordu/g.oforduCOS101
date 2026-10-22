use std::io::Write;

fn main() {
    let l = ["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let s = ["Legend", "Turbo King", "Williams", "Fayrouz"];
    let n = ["Maltina", "Amstel Malta", "Malta Gold"];

    let mut file = std::fs::File::create("nigerian_breweries.txt").expect("Failed to create file");

    file.write_all(b"NIGERIAN BREWERIES PLC - DRINKS CATEGORIES\n").expect("Write failed");
    file.write_all(b"*****************************************\n").expect("Write failed");
    file.write_all(b"\n").expect("Write failed");
    file.write_all(b"Lager | Stout | Non Alcoholic\n").expect("Write failed");
    file.write_all(b"-----------------------------------------\n").expect("Write failed");

    for i in 0..6 {
        file.write_all(l[i].as_bytes()).expect("Write failed");
        file.write_all(b" | ").expect("Write failed");
        
        let stout = s.get(i).unwrap_or(&"");
        file.write_all(stout.as_bytes()).expect("Write failed");
        file.write_all(b" | ").expect("Write failed");
        
        let non_alc = n.get(i).unwrap_or(&"");
        file.write_all(non_alc.as_bytes()).expect("Write failed");
        
        file.write_all(b"\n").expect("Write failed");
    }
}
