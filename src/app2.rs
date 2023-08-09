use leptos::prelude::*;
use leptos::IntoProperty;
use leptos::IntoView;
use leptos::Scope;
use leptos::Property;
use leptos::View;
use leptos::component;

struct ChurnRate {
    churn_rate: f64,
}

impl IntoProperty for ChurnRate {
    fn into_property(&self, scope: leptos::Scope) -> Property {
        Property::from_str(&self.churn_rate)
    }
}

impl IntoView for ChurnRate {
    fn into_view(&self, scope: leptos::Scope) -> View {
        View::new(format!("Churn Rate: {}", self.churn_rate))
    }
}

fn churn_rate(customers_start: i64, customers_end: i64, new_customers: i64) -> String {
    let total_customers = customers_start + new_customers;
    let churned_customers = customers_start - customers_end;
    return format!("Churn Rate: {}", churned_customers as f64 / total_customers as f64).to_string();
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    // Creates a `ChurnRate` struct
    let churn_rate = ChurnRate { churn_rate: churn_rate(customers_start, customers_end, new_customers) };

    // Updates the button text with the churn rate
    let on_churn_input = move |ev| {
        // event_target_value is a Leptos helper function
        // it functions the same way as event.target.value
        // in JavaScript, but smooths out some of the typecasting
        // necessary to make this work in Rust
        set_churn_rate(churn_rate);
    };

    view! { cx,
        <h1>"Welcome to Profiteer"</h1>
        <h3>"Instructions"</h3>
        <p>"Enter the appropriate values below and then click calculate to see your metrics."</p>
        <h2>"Customer Churn"</h2>
        <p>"Customers at start of period"</p>
        <input type="text" customers_start="customers_start"></input>
        <p>"Costumers at end of period"</p>
        <input type="text" customers_end="customers_end"></input>
        <p>"New Customers during period"</p>
        <input type="text" new_customers="new_customers"></input>
        <h1><button on:click=on_churn_input>"Calculate"</button></h1>
        <h1>"Churn Rate"</h1>
        <input type="text"
               on:input=on_churn_input
               prop:value=churn_rate>
        </input>
    }
}

