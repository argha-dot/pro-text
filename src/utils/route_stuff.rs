use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
pub struct HomePageQueries {
    pub user: Option<String>,
}

#[derive(Params, PartialEq)]
pub struct UserPageParams {
    pub username: String,
}
