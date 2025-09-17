mod constants;
mod routes;

use constants::apiurl::api_url;
use routes::app::App;
use routes::blog::Blog;
use routes::post::Post;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/blog/:slug")]
    Post { slug: String },
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <App /> },
        Route::Blog => html! { <Blog api={api_url()} /> },
        Route::Post { slug } => {
            html! { <Post slug={slug} api={api_url()} /> }
        }
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
