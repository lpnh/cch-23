use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct ParsedPath(pub Vec<i32>);

#[async_trait]
impl<S> FromRequestParts<S> for ParsedPath
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut nums = Vec::new();

        for segment in req.uri.path().trim_start_matches("/1/").split('/') {
            match segment.parse::<i32>() {
                Ok(num) => nums.push(num),
                Err(_) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        format!("Invalid segment: {}", segment),
                    ))
                }
            }
        }

        Ok(ParsedPath(nums))
    }
}
