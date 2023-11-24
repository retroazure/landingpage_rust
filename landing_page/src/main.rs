use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <Header/> })
}

//This is the Main Header of the page
#[component]
fn Header() -> impl IntoView {
    view! {
        <div class=("main_header")>
        <h1>Your Gateway To Decentralized Finance</h1>

        <h4>AirDao Is A Community-Governed Layer One Blockchain And Ecosystem Of Web3 Dapps. Powered By The AMB Token</h4>
        </div>

        <div class="button_div">
        <button class="button_style">Trade AMB on firepot</button>
        <button class="button_style">Use AMB Bridge</button>
    </div>
    }
}
