pub fn parse(datas: &str) -> Result<(), serde_json::Error> {
    let data = datas;

    #[derive(serde::Deserialize)]
    struct Person {
        name: String,
        age: u8,
        phones: Vec<String>,
    }

    let p: Person = serde_json::from_str(data)?;

    println!("Please call {} the age are {} at the number {}", p.name, p.age,p.phones[0]);

    Ok(())

}