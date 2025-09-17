use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="max-w-xl">
            <div class="flex items-center gap-2">
                <img class="w-10 h-10" src="https://github.com/myferr.png" alt="github.com/myferr" />
                <a class="font-bold text-lg text-[#e5e5e5]" href="/">{"myfer"}</a>
            </div>
            <p>
                {"hey there! i build small things with "}
                <span class="text-[#e5e5e5] underline">{"rust"}</span>
                {", "}
                <span class="text-[#e5e5e5] underline">{"go"}</span>
                {", and "}
                <span class="text-[#e5e5e5] underline">{"ts"}</span>
                {". "}
            </p>
            <p>
                {"i'm mostly in "}
                <span class="text-[#e5e5e5] underline">{"zed"}</span>
                {", hacking on "}
                <span class="text-[#e5e5e5] underline">{"websites"}</span>
                {" and "}
                <span class="text-[#e5e5e5] underline">{"clis"}</span>
                {". "}
            </p>
            <br/>
            <p>
                {"currently living in macos or arch linux."}
            </p>
            <br/>
            <div class="flex items-center gap-6">
                <a href="https://github.com/myferr" target="_blank" class="text-[#e5e5e5] underline">
                    {"github"}
                </a>
                <a href="https://x.com/myferdoescoding" target="_blank" class="text-[#e5e5e5] underline">
                    {"x dot com"}
                </a>
                <a href="/blog" class="text-[#e5e5e5] underline">
                    {"blog"}
                </a>
            </div>
        </main>
    }
}
