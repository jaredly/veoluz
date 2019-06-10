use yew::prelude::*;

struct Model {
    config: shared::Config,
}

enum Msg {
    Reset(shared::Config),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut config = shared::Config::new(vec![], 0, 0);
        crate::state::with(|state| {
          config = state.config;
          state.callbacks.push(link.send_back(|c| Reset(c)))
        });
        Self {
            value: config,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Reset(config) => self.value = config
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
               <button onclick=|_| Msg::DoIt,>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}