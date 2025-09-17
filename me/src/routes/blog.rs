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

#[derive(Properties, PartialEq)]
pub struct BlogProps {
    pub api: String,
}

#[function_component(Blog)]
pub fn blog(props: &BlogProps) -> Html {
    let api_url = props.api.clone();
    let posts = use_state(Vec::new);
    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            let posts = posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let api_url = api_url.clone();
                if let Ok(response) = Request::get(&format!("{}/posts", api_url)).send().await
                    && let Ok(fetched_posts) = response.json::<Vec<Post>>().await
                {
                    posts.set(fetched_posts);
                }
            });
            || ()
        });
    }

    html! {
        <div class="flex min-h-screen bg-white text-black/70 dark:text-[#a1a1a1] dark:bg-[#0a0a0a]">
            <aside class="hidden md:flex md:flex-col md:w-64 bg-gray-200 dark:bg-neutral-900 p-6">
                <div class="flex items-center gap-2 mb-6">
                    <img class="w-10 h-10" src="https://github.com/myferr.png" alt="github.com/myferr" />
                    <a class="font-bold text-lg text-black dark:text-[#e5e5e5]" href="/">{"myfer"}</a>
                </div>
                <nav class="flex flex-col gap-4">
                    <a href="https://github.com/myferr" target="_blank" class="text-black dark:text-[#e5e5e5] underline">{"github"}</a>
                    <a href="https://x.com/myferdoescoding" target="_blank" class="text-black dark:text-[#e5e5e5] underline">{"x dot com"}</a>
                    <a href="/blog" class="text-black dark:text-[#e5e5e5] underline">{"blog"}</a>
                </nav>
            </aside>

            <main class="flex-1 p-6 max-w-3xl">
                <h1 class="text-2xl font-bold text-black dark:text-[#e5e5e5]">{"blog"}</h1>

                <p>
                    {"welcome to "}
                    <span class="text-black dark:text-[#e5e5e5] underline">{"my blog"}</span>
                    {" where i mostly talk about programming."}
                </p>

                <br/>

                <h2 class="text-xl font-bold text-black dark:text-[#e5e5e5]">
                    {"posts"}
                </h2>

                <ul class="mt-4">
                    { for posts.iter().map(|post| html! {
                        <li class="mb-2">
                            <span>{"- "}</span>
                            <Link<Route> to={Route::Post { slug: post.slug.clone() }}>
                                <span class="text-black dark:text-[#e5e5e5] underline">{ &post.title }</span>
                            </Link<Route>>
                        </li>
                    })}
                </ul>
                <p>
                    {"end of posts."}
                </p>
                <br />
                <Link<Route> to={Route::Home}>
                    <span class="text-black dark:text-[#e5e5e5] underline">{"go home"}</span>
                </Link<Route>>
            </main>
        </div>
    }
}
