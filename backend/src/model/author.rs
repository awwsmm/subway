use uuid::Uuid;

pub(crate) struct Author {
    id: Uuid, // note: Uuid implements Copy
    name: String,
}

// We add a new() function to avoid
//  - exposing the 'id' field to mutation
//  - users creating 'name's of unbounded length
impl Author {
    pub(crate) fn new(name: String) -> Self {

        // TODO add name validation

        Self {
            id: Uuid::new_v4(),
            name,
        }
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}