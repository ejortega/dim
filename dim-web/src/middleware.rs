use axum::extract::State;
use dim_core::errors::DimError;
use dim_database::DbConnection;

use crate::DimErrorWrapper;

pub async fn verify_cookie_token<B>(
    State(conn): State<DbConnection>,
    mut req: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<axum::response::Response, DimErrorWrapper> {
    match req.headers().get(axum::http::header::AUTHORIZATION) {
        Some(token) => {
            let mut tx = match conn.read().begin().await {
                Ok(tx) => tx,
                Err(_) => {
                    return Err(DimErrorWrapper(DimError::DatabaseError {
                        description: String::from("Failed to start transaction"),
                    }))
                }
            };
            let id = dim_database::user::Login::verify_cookie(token.to_str().unwrap().to_string())
                .map_err(DimError::CookieError)
                .map_err(DimErrorWrapper)?;

            let current_user = dim_database::user::User::get_by_id(&mut tx, id)
                .await
                .map_err(|_| DimError::UserNotFound)
                .map_err(DimErrorWrapper)?;

            req.extensions_mut().insert(current_user);
            Ok(next.run(req).await)
        }
        None => Err(DimErrorWrapper(DimError::NoToken)),
    }
}
