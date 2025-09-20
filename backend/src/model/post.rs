use uuid::Uuid;

pub(crate) struct Post {
    id: Uuid, // note: Uuid implements Copy
    title: String,
}

// We add a new() function to avoid
//  - exposing the 'id' field to mutation
//  - users creating 'title's of unbounded length
impl Post {
    pub(crate) fn new(title: String) -> Self {

        // TODO add title validation

        Self {
            id: Uuid::new_v4(),
            title,
        }
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }
    
    pub(crate) fn title(&self) -> &str {
        &self.title
    }
}