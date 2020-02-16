#[cfg(target_arch = "wasm32")]
fn main() {
    yew::start_app::<frontendyew::Model>();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
