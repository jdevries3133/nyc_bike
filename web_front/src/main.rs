use lib::constants;
use reqwasm::http::Request;
use wasm_bindgen_futures;
use wasm_logger;
use yew::prelude::*;

struct FunctionComponentState {
    value: String,
}

#[function_component(FunctionComponent)]
fn function_component() -> Html {
    let data = use_state(|| None);
    {
        let data = data.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_data = Request::get(&constants::get_backend_base_url())
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    data.set(Some(FunctionComponentState {
                        value: format!("{}", fetched_data),
                    }));
                });
                || ()
            },
            (),
        );
    }
    match &(*data) {
        Some(s) => html! {
            <p>{ format!("{}", s.value) }</p>
        },
        None => html! {
            <p>{ "loading... " }</p>
        },
    }
}

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class="w-full min-h-screen flex flex-col items-center justify-center">
                <button
                    class="bg-green-100 p-2 m-2 font-bold rounded shadow hover:bg-green-200 hover:shadow-none"
                    onclick={link.callback(|_| Msg::AddOne)}
                >{ "+1" }</button>
                <p class="text-xl serif">{ "Current Count: " }{ self.value }</p>
                <FunctionComponent />
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
