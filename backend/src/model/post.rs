use std::ops::Deref;
use uuid::Uuid;

/// Newtype representing the post's UUID.
pub(crate) struct PostId(Uuid);

// We impl only Deref, not DerefMut
impl Deref for PostId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Newtype representing the post author's UUID.
pub(crate) struct AuthorId(Uuid);

impl Deref for AuthorId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AuthorId {
    pub(crate) fn new(id: Uuid) -> Self {
        Self(id)
    }
}

/// Newtype representing the post title.
pub(crate) struct Title(String);

impl Deref for Title {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Title {
    pub(crate) fn new(title: String) -> Self {
        Self(title)
    }
}

/// Newtype representing the post body.
pub(crate) struct Body(String);

impl Deref for Body {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Body {
    pub(crate) fn new(body: String) -> Self {
        Self(body)
    }
}

pub(crate) struct Post {
    post_id: PostId,
    author_id: AuthorId,
    title: Title,
    body: Body,
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

    pub(crate) fn post_id(&self) -> &PostId {
        &self.post_id
    }

    pub(crate) fn author_id(&self) -> &AuthorId {
        &self.author_id
    }

    pub(crate) fn title(&self) -> &Title {
        &self.title
    }

    pub(crate) fn body(&self) -> &Body {
        &self.body
    }
}