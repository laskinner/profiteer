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
            <nav class="bg slate-900">
              <div class="container mx-auto px-4">
                <div class="flex items-center justify-between yf-16">
                  <div class="flex items-center">
                    <a href="www.google.com" class="text-slate-900 font-bold text-lg">"Profiteer"</a>
                  </div>
                  <div class="flex items-center">
                    <a href="www.google.com" class="text-white hover:text-gray-300 px-3 py-2">"Home"</a>
                    <a href="www.facebook.com" class="text-white hover:text-gray-300 px-3 py-2">"About"</a>
                    <a href="www.bing.com" class="text-white hover:text-gray-300 px-3 py-2">"Contact"</a>
                  </div>
                </div>
              </div>
            </nav>
            
       // sets the document title
        <Title text="Profiteer"/>

        <body class="bg-background-50">
            // content for this welcome page
            <h2 class="text-center text-xl text-slate-900 pb-10 pt-10">"Instructions: Provide values for the business period."</h2>
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
        <main>
            <div class="grid grid-cols-2 gap-4">
                <div class="mp-6 p-6 mx-auto rounded-xl items-center space-x-4 text-slate-900">

                    <p class="text-2xl text-center">"Beginning of period"</p>

                    <div class="flex float-right">
                        <label for="input" class="mr-2">"Number of customers:"</label>
                            <input type="text" class="flex-shrink rounded border text-white bg-indigo-300" on:input=move |ev| {
                            set_customers_start(event_target_value(&ev).parse::<i32>().unwrap());
                            } prop:value=customers_start/>
                    </div>

                    <p class="text-2xl text-center">"End of period"</p>
                    
                    <div class="flex float-right">
                        <label for="input" class="mr-2">"Number of Customers:"</label>
                            <input type="text" class="flex-shrink rounded border text-white bg-indigo-300" on:input=move |ev| {
                            set_customers_end(event_target_value(&ev).parse::<i32>().unwrap());
                            } prop:value=customers_end/>
                    </div>
                    
                    <p class="text-2xl text-center">"During the period"</p>

                    <div class="flex float-right">
                        <label for="input" class="mr-2">"Customers added:"</label>
                            <input type="text" class="flex-shrink rounded border text-white bg-indigo-300" on:input=move |ev| {
                            set_customers_added(event_target_value(&ev).parse::<i32>().unwrap());
                            } prop:value=customers_added/>
                    </div>
            </div>
            <div class="mp-6 mx-auto rounded-xl items-center space-x-4 text-white bg-indigo-600">
                <ul class="p-6 divide-y divide-slate-200 text-right"> 
                    <p>"Current Customers: " {customers_end}</p>
                    <p>"New Customers: " {customers_added}</p>
                    <p>"Total Customers Lost: " {total_customers_lost}</p>
                    <p>"Net Customers Lost: " {net_customers_lost}</p>
                    <p>"Churn rate: " {churn}"%"</p>
                </ul>
            </div>
            </div>
        </main>
        }   
}
