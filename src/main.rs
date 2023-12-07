use leptos::*;
use log::info;
use surrealdb::{Surreal, engine::remote::ws::Ws};

async fn get_user(user: String) -> Result<String, ServerFnError> {
    let now = instant::Instant::now();
    
    // this await stalls for 5 seconds before the result is returned
    let _db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // for me this prints "took 5055ms to connect"
    info!("took {}ms to connect", now.elapsed().as_millis());

    // using the db after it has connected is fast

    Ok(format!("this user is {user}"))
}

#[component]
fn App() -> impl IntoView {
    
    let user = create_resource(|| (), |_| async { get_user("john".into()).await });
    view! {
        <Suspense fallback=move || view! {<p>"LOADING"</p> }>
            <ErrorBoundary fallback=|_| {view! {<p>"Something went wrong"</p>}}>
                {move || {
                    user.get().map(move |x| {
                        x.map(move |y| {
                            view! {<p>"User: "{y}</p>}
                        })
                    })
                }}
            </ErrorBoundary>
        </Suspense>
    }
}

fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);

    leptos::mount_to_body(|| view! { <App/> })
}