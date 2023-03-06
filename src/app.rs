use yew::prelude::*;
use yew::{function_component, html, Html, Properties, Callback};

#[function_component(App)]
pub fn app() -> Html {

    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
    });

    html! {
        <main>
            <h1>{"astrid's animal store"}</h1>
            <div>
                <h2>{"animals for sale"}</h2>
            </div>
            <button {onclick}>
                { "buy an animal"}
                <a href="mouse.png" download="mouse.png">{"PLEASE work IM CRYING"}</a>
            </button>

            /* 
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        
            <>
                <h1>{ "RustConf Explorer" }</h1>
                <div>
                    <h3>{"Videos to watch"}</h3>
                    <p>{ "John Doe: Building and breaking things" }</p>
                    <p>{ "Jane Smith: The development process" }</p>
                    <p>{ "Matt Miller: The Web 7.0" }</p>
                    <p>{ "Tom Jerry: Mouseless development" }</p>
                </div>
                <div>
                    <h3>{ "John Doe: Building and breaking things" }</h3>
                    <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
                </div>
            </>
        */
        </main>
    }
}
