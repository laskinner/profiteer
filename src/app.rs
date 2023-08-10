use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Profiteer"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    
    let (customers_start, set_customers_start) = create_signal(cx, "Enter number".to_string());
    let (customers_end, set_customers_end) = create_signal(cx, "Enter number".to_string());
    let (customers_added, set_customers_added) = create_signal(cx, "Enter number".to_string());
    
    view! { cx,
        <h1>"Welcome to Profiteer"</h1>
         <form><label>"Enter the number of customers at the start of the period "
            <input type="text"
        on:input=move |ev| {
            // event_target_value is a Leptos helper function
            // it functions the same way as event.target.value
            // in JavaScript, but smooths out some of the typecasting
            // necessary to make this work in Rust
            set_customers_start(event_target_value(&ev));
        }

        // the `prop:` syntax lets you update a DOM property,
        // rather than an attribute.
        prop:value=customers_start
    />
            </label></form>
        <form><label>"Enter the number of customers at the en of the period "
            <input type="text"
        on:input=move |ev| {
            // event_target_value is a Leptos helper function
            // it functions the same way as event.target.value
            // in JavaScript, but smooths out some of the typecasting
            // necessary to make this work in Rust
            set_customers_end(event_target_value(&ev));
        }

        // the `prop:` syntax lets you update a DOM property,
        // trather than an attribute.
        prop:value=customers_end
    /></label></form>
<form><label>"Enter the number of customers added during the period "
            <input type="text"
        on:input=move |ev| {
            // event_target_value is a Leptos helper function
            // it functions the same way as event.target.value
            // in JavaScript, but smooths out some of the typecasting
            // necessary to make this work in Rust
            set_customers_added(event_target_value(&ev));
        }

        // the `prop:` syntax lets you update a DOM property,
        // rather than an attribute.
        prop:value=customers_added
    /></label></form>
 <p>"Number of Customers at start of period: " {customers_start}</p>
 <p>"Number of Customers at end of period: " {customers_end}</p>
 <p>"Number of Customers added during period: " {customers_added}</p>
}   
}





