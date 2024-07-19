use crate::tiktok::api_error::ApiError;
use crate::tiktok::user_error::UserError;
pub enum ErrorType {
    Backend(ApiError),
    User(UserError)
}