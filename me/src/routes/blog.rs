use crate::Route;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct Post {
    slug: String,
    title: String,
}

#[function_component(Blog)]
pub fn blog() -> Html {
    let posts = use_state(Vec::new);
    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            let posts = posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(response) = Request::get("http://localhost:3000/posts").send().await
                    && let Ok(fetched_posts) = response.json::<Vec<Post>>().await
                {
                    posts.set(fetched_posts);
                }
            });
            || ()
        });
    }

    html! {
        <main class="max-w-xl">
            <h1 class="text-2xl font-bold text-[#e5e5e5]">{"Blog"}</h1>
            <ul class="mt-4">
                { for posts.iter().map(|post| html! {
                    <li class="mb-2">
                        <Link<Route> to={Route::Post { slug: post.slug.clone() }}>
                            <span class="text-[#e5e5e5] underline">{ &post.title }</span>
                        </Link<Route>>
                    </li>
                })}
            </ul>
            <br />
            <Link<Route> to={Route::Home}>
                <span class="text-[#e5e5e5] underline">{"home"}</span>
            </Link<Route>>
        </main>
    }
}
