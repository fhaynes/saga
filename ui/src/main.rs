#![recursion_limit="128"]
extern crate chrono;
#[macro_use]
extern crate yew;

use yew::html::*;

mod layout;

struct Model;

enum Msg {
}

fn update(_: &mut Context<Msg>, model: &mut Model, msg: Msg) {
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div class=("mdl-layout", "mdl-js-layout", "mdl-layout--fixed-header"),>
        <header class="mdl-layout__header",>
            <div class="mdl-layout__header-row",>
            <span class="mdl-layout-title",>{ "Saga" }</span>
            <div class="mdl-layout-spacer",></div>
            <nav class=("mdl-navigation", "mdl-layout--large-screen-only"),>
                <a class="mdl-navigation__link", href="",>{ "Menu A" }</a>
                <a class="mdl-navigation__link", href="",>{ "Menu B" }</a>
            </nav>
            </div>
        </header>
        <div class="mdl-layout__drawer",>
            <span class="mdl-layout-title",>{ "Navigation" }</span>
            <nav class="mdl-navigation",>
            <a class="mdl-navigation__link", href="",>{ "Nodes" }</a>
            <a class="mdl-navigation__link", href="",>{ "Indices" }</a>
            </nav>
        </div>
        <main class="mdl-layout__content",>
            <div class="page-content",></div>
        </main>
        </div>
    }
}

fn main() {
    let model = Model {};
    program(model, update, view);
}