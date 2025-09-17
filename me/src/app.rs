use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2">
                        <img class="w-10 h-10" src="https://github.com/myferr.png" alt="github.com/myferr" />
                        <a class="font-bold text-lg text-white" href="/">{"myfer"}</a>
                    </div>
                    <div class="flex gap-2">
                        <a class="text-white underline" href="https://github.com/myferr">{"github"}</a>
                        <a class="text-white underline" href="https://x.com/myferdoescoding">{"x dot com"}</a>
                    </div>
            </div>
        </main>
    }
}
