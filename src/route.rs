use dioxus::prelude::*;

use crate::components::routes::Home;
use crate::components::routes::Login;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Login,
    #[route("/home")]
    Home,
}
