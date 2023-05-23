#[derive(Debug, serde_tag::WithTag, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[tag(tag = "user")]
pub struct User {
    pub name: String,
    pub age: u8,
}

fn main() {
    let user = User {
        name: "u".into(),
        age: 33,
    };
    println!("{:#?}", user);
    println!("{:#?}", user.into_tagged());
}