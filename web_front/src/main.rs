use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

#[function_component(FunctionComponent)]
fn function_component() -> Html {
    html! {
        <p>{ "This is a functional component" }</p>
    }
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
    yew::start_app::<Model>();
}
