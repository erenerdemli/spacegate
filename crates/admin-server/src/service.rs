use axum::{middleware, Router};
use spacegate_config::service::*;

use crate::{mw, state::AppState};

pub mod config;
pub mod plugin;
pub fn router<B>(state: AppState<B>) -> Router<()>
where
    B: Create + Retrieve + Update + Delete + Send + Sync + 'static,
{
    Router::new()
        .nest(
            "/config",
            config::router::<B>().layer(middleware::from_fn_with_state(state.clone(), mw::version_control::version_control)),
        )
        .nest("/plugin", plugin::router::<B>())
        .with_state(state)
}
