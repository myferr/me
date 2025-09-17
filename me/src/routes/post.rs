use crate::Route;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub slug: String,
    pub api: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let post_html = use_state(String::new);
    {
        let post_html = post_html.clone();
        use_effect_with(
            (props.slug.clone(), props.api.clone()),
            move |(slug, api)| {
                let post_html = post_html.clone();
                let slug = slug.clone();
                let api = api.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(response) = Request::get(&format!("{}/posts/{}", api, slug))
                        .send()
                        .await
                        && let Ok(fetched_html) = response.text().await
                    {
                        post_html.set(fetched_html);
                    }
                });
                || ()
            },
        );
    }

    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&post_html);
    div.set_attribute("class", "notailwind").unwrap();

    html! {
        <main class="max-w-xl prose prose-invert">
            { Html::VRef(div.into()) }
            <br />
            <Link<Route> to={Route::Blog}>
                <span class="text-[#e5e5e5] underline">{"back to blog"}</span>
            </Link<Route>>
        </main>
    }
}
