use gloo_net::http::Request;
use yew::prelude::*;

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
    div.set_attribute("class", "max-w-xl notailwind").unwrap();

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
                { Html::VRef(div.into()) }
            </main>
        </div>
    }
}
