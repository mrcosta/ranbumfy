pub struct Artist {
    pub name: String,
    pub id: String,
    pub albums: Vec<Album>,
}

#[derive(Debug)]
pub struct Album {
    pub name: String,
    pub id: String,
    pub url: String,
}
