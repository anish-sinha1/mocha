mod auth;
mod pagination;
mod posts;
mod types;
mod users;

pub use pagination::{PaginationLimits, SpacePaginationOptions, TagPaginationOptions};
pub use posts::{
    CreatePost, CreateSpace, CreateTag, DeletePost, DeleteSpace, DeleteTag, EditPost, EditSpace,
    EditTag, GetPostById, GetPostsByUser, GetSpaceById, GetTagById, GetTagsBySpace,
};
pub use types::HashAlgorithm;
pub use users::{CreateUser, DeleteUser, EditUser, EditUserInfo, GetUserByEmail, GetUserById};

pub use auth::{
    CreateSession, DeleteSession, DeleteUserSessions, EditSession, GetSessionById,
    GetSessionsByUserId, LoginUser, RegisterUser, UpdateSessionState,
};
