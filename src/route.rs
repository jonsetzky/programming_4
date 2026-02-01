use dioxus::prelude::*;

use crate::AppState;
use crate::components::routes::Home;
use crate::components::routes::Login;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Login,
    #[route("/home")]
    Home,
    // #[route("/user/:id")]
    // User { id: u32 },
}
