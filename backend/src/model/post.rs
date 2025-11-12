use uuid::Uuid;

// This type is public, but the field is private, so it cannot be constructed manually.
// This is intentional -- it forces the user to use the new() method to create a Post.
pub(crate) struct PostId(Uuid);

pub(crate) struct AuthorId(pub(crate) Uuid);
pub(crate) struct Title(pub(crate) String);
pub(crate) struct Body(pub(crate) String);

// We use newtypes here, so that a UUID post_id and a UUID author_id cannot be swapped accidentally
pub(crate) struct Post {
    pub(crate) post_id: PostId,
    pub(crate) author_id: AuthorId,
    pub(crate) title: Title,
    pub(crate) body: Body,
}

// We add a new() function to avoid
//  - exposing the 'id' field to mutation
//  - users creating 'title's of unbounded length
impl Post {
    pub(crate) fn new(
        author_id: AuthorId,
        title: Title,
        body: Body,
    ) -> Self {

        // TODO add title validation

        Self {
            post_id: PostId(Uuid::new_v4()),
            author_id,
            title,
            body,
        }
    }
}