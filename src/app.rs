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
    // Churn Rate 
    
    let (customers_start, set_customers_start) = create_signal(cx, 1);
    let (customers_end, set_customers_end) = create_signal(cx, 0);
    let (customers_added, set_customers_added) = create_signal(cx, 0);
    let churn = move || (customers_start() - customers_end()) / customers_start() * 100;
    
    // Revenue Churn
    
    view! { cx,
        <h1>"Welcome to Profiteer"</h1>
        
        <form><label>"Enter the number of customers at the start of the period "
            <input type="text" on:input=move |ev| {
            set_customers_start(event_target_value(&ev).parse::<i32>().unwrap());
        } prop:value=customers_start
        />
        </label></form>
        
        <form><label>"Enter the number of customers at the end of the period "
            <input type="text" on:input=move |ev| {
            set_customers_end(event_target_value(&ev).parse::<i32>().unwrap());
        } prop:value=customers_end
        />
        </label></form>
    
        <form><label>"Enter the number of customers added during the period "
            <input type="text" on:input=move |ev| {
            set_customers_added(event_target_value(&ev).parse::<i32>().unwrap());
        } prop:value=customers_added
        />
        </label></form>
 
        <p>"Number of Customers at start of period: " {customers_start}</p>
        <p>"Number of Customers at end of period: " {customers_end}</p>
        <p>"Number of Customers added during period: " {customers_added}</p>


        <p>"Churn rate: " {churn}"%"</p>
    }   
}
