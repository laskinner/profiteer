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

        <body class="bg-gray-900/90">
            // content for this welcome page
        <h1 class="text-center text-xl text-white">"Welcome to Profiteer"</h1>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
        </body>
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
        <main class="p-6 max-w-lg mx-auto rounded-xl shadow-lg items-center space-x-4 text-lime-400 bg-violet-600/50">
            <div class="text-center">
                <h2>"Instructions: Provide values for the business period."</h2>
                <h2>"Beginning of period"</h2>

                    <label>"Number of customers "
                        <input type="text" on:input=move |ev| {
                        set_customers_start(event_target_value(&ev).parse::<i32>().unwrap());
                        } prop:value=customers_start/>
                    </label>

                <h2>"End of period"</h2>
                
                    <label>"Number of Customers "
                        <input type="text" on:input=move |ev| {
                        set_customers_end(event_target_value(&ev).parse::<i32>().unwrap());
                        } prop:value=customers_end/>
                    </label>
                
                <h2>"During the period"</h2>

                    <label>"Customers added "
                        <input type="text" on:input=move |ev| {
                        set_customers_added(event_target_value(&ev).parse::<i32>().unwrap());
                        } prop:value=customers_added/>
                    </label>
           </div> 
            <ul class="p-6 divide-y divide-slate-200"> 
                <p>"Current Customers: " {customers_end}</p>
                <p>"New Customers: " {customers_added}</p>
                <p>"Total Customers Lost: " {total_customers_lost}</p>
                <p>"Net Customers Lost: " {net_customers_lost}</p>
                <p>"Churn rate: " {churn}"%"</p>
            </ul>
         
        </main>
    }   
}
