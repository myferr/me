mod routes;

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
        Route::Blog => html! { <Blog /> },
        Route::Post { slug } => {
            html! { <Post slug={slug} api={option_env!("API_URL").unwrap_or("http://localhost:3000").to_string()} /> }
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
