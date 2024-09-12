


struct DbItem {
    text_data: String, // TODO: make it a str?
    vector_data: Box([f32])
}


impl DbItem {
    pub fn new(text_data: String) -> DbItem {
        let vector_data = [0f32, 0f32, 0f32, 0f32, 0f32];
        Self {
            text_data,
            vector_data: Box::new(*vector_data)
        }
    }

    pub fn compare(&self, other: &DbItem) -> f32 {
        0.0
    }
}


struct VectorDBCore {
    data: Vec<DbItem>,

}

/*
Currently:
  Unsorted List:
    Let each vector have (k) elements
    Insert: O(1)
    Search: O(n * k)

*/
impl VectorDBCore {
    pub fn new() -> VectorDBCore {
        let phony_data: String = "".to_string();
        Self {
            data: vec![DbItem::new(phony_data)]
        }
    }

    pub fn add_item(&mut self, text_item: &String) {
        self.data.push(DbItem::new(text_item.clone()));
    }


}




