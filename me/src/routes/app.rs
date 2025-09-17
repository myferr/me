use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="flex min-h-screen bg-white text-black/50 dark:text-[#a1a1a1] dark:bg-[#0a0a0a]">
            <aside class="hidden md:flex md:flex-col md:w-64 bg-gray-200 dark:bg-neutral-900 p-6">
                <div class="flex items-center gap-2 mb-6">
                    <img class="w-10 h-10" src="https://github.com/myferr.png" alt="github.com/myferr" />
                    <a class="font-bold text-lg text-black dark:text-[#e5e5e5]" href="/">{"myfer"}</a>
                </div>
                <nav class="flex flex-col gap-4">
                    <a href="https://github.com/myferr" target="_blank" class="dark:text-[#e5e5e5] text-[#0a0a0a] underline">{"github"}</a>
                    <a href="https://x.com/myferdoescoding" target="_blank" class="dark:text-[#e5e5e5] text-[#0a0a0a] underline">{"x dot com"}</a>
                    <a href="/blog" class="dark:text-[#e5e5e5] text-[#0a0a0a] underline">{"blog"}</a>
                </nav>
            </aside>

            <main class="flex-1 p-6 max-w-3xl">
                <p>
                    {"hey there! i build small things with "}
                    <span class="dark:text-[#e5e5e5] text-[#0a0a0a] underline">{"rust"}</span>
                    {", "}
                    <span class="dark:text-[#e5e5e5] text-[#0a0a0a] underline">{"go"}</span>
                    {", and "}
                    <span class="dark:text-[#e5e5e5] text-[#0a0a0a] underline">{"ts"}</span>
                    {". "}
                    </p>
                    <p>
                    {"i'm mostly in "}
                    <span class="dark:text-[#e5e5e5] text-[#0a0a0a] underline">{"zed"}</span>
                    {", hacking on "}
                    <span class="dark:text-[#e5e5e5] text-[#0a0a0a] underline">{"websites"}</span>
                    {" and "}
                    <span class="dark:text-[#e5e5e5] text-[#0a0a0a] underline">{"clis"}</span>
                    {". "}
                    <br/>
                    <br/>
                    <p>
                    {"currently living in macos or arch linux."}
                    </p>
                </p>
            </main>
        </div>
    }
}
