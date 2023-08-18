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
    let (customers_end, set_customers_end) = create_signal(cx, 1);
    let (customers_added, set_customers_added) = create_signal(cx, 1);
    let total_customers_lost = move || customers_start() - customers_end() + customers_added();
    let net_customers_lost = move || customers_start() - customers_end();
    let churn = move || net_customers_lost() as f32 / customers_start() as f32 * 100.0;
    
    // Revenue Churn
    
    view! { cx,
         <main class="my-0 mx-auto max-w-3xl text-center">
        <h1>"Welcome to Profiteer"</h1>
            </main>
<article class="prose lg:prose-xl">
        
        <h2>"Instructions: Provide values for the business period."</h2>
</article>        
        <h2>"Beginning of period"</h2>
        
        <form><label>"Number of customers "
            <input type="text" on:input=move |ev| {
            set_customers_start(event_target_value(&ev).parse::<i32>().unwrap());
        } prop:value=customers_start
        />
        </label></form>

        <h2>"End of period"</h2>
        
        <form><label>"Number of Customers "
            <input type="text" on:input=move |ev| {
            set_customers_end(event_target_value(&ev).parse::<i32>().unwrap());
        } prop:value=customers_end
        />
        </label></form>
    
        <h2>"During the period"</h2>

        <form><label>"Customers added "
            <input type="text" on:input=move |ev| {
            set_customers_added(event_target_value(&ev).parse::<i32>().unwrap());
        } prop:value=customers_added
        />
        </label></form>
 
        <p>"Current Customers: " {customers_end}</p>
        <p>"New Customers added: " {customers_added}</p>
        <p>"Total Customers Lost: " {total_customers_lost}</p>
        <p>"Net Customers Lost: " {net_customers_lost}</p>
        <p>"Churn rate: " {churn}"%"</p>
    }   
}
