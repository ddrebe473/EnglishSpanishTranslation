use yew::prelude::*;
use crate::text_input::TextInput;

use crate::translate::translate_phrase;

pub enum Msg {
    Setphrase(String),
    // Regeneratephrase,
}

#[derive(Debug, Default)]
pub struct App {
    phrase: String,
}

impl App {
    fn get_translate(&self) -> String {
        String::from(translate_phrase(&self.phrase))
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Setphrase(next_phrase) => self.phrase = next_phrase,
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(Msg::Setphrase);
        html! {
            <main>
                <div class="entry">
                    <div class="text">
                        {"Enter a phrase below:"}
                    </div>
                    
                    <div class="input">
                        <TextInput {on_change} value={self.phrase.clone()} />
                    </div>
                </div>
                <div class="readout">
                    <div class="get-translation">
                        {self.get_translate()}
                     </div>
                </div>
            </main>
        }
    }
}