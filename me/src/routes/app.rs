use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="flex min-h-screen bg-white text-black/50 dark:text-[#a1a1a1] dark:bg-[#0a0a0a]">
            <aside class="hidden md:flex md:flex-col md:w-64 bg-gray-200 dark:bg-neutral-900 p-6 border-r border-gray-200 dark:border-neutral-700">
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
                </p>

                <br/>

                <p>
                {"currently living in macos or arch linux."}
                </p>

                <br/>

                <p>
                {" cool people → "}
                </p>
                <p>
                {"here are some pretty "}
                <span class="dark:text-[#e5e5e5] text-[#0a0a0a] underline">{"cool people"}</span>
                {", go checkout their stuff"}
                </p>
                <div class="flex flex-wrap gap-1">
                    <a href="https://github.com/notaussie" target="_blank">
                        <img src="https://github.com/notaussie.png" alt="notaussie" width="32" height="32"/>
                    </a>
                    <a href="https://github.com/veddevv" target="_blank">
                        <img src="https://github.com/veddevv.png" alt="veddevv" width="32" height="32"/>
                    </a>
                    <a href="https://goobin.club" target="_blank">
                        <img src="https://github.com/qrunk.png" alt="qrunk" width="32" height="32"/>
                    </a>
                    <a href="https://git.swee.codes/swee" target="_blank">
                        <img src="https://github.com/Sweeistaken.png" alt="sweeistaken" width="32" height="32"/>
                    </a>
                </div>
            </main>
        </div>
    }
}
