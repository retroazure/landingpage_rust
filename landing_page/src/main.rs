use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { 
        <Navbar />
        <Header/>
        <Section />
     })
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

#[component]
fn Section() -> impl IntoView {
    view! {
        <section class=("main_section")>
        <div class=("div_section")>
            <h1>Buy AMB On Binance</h1>
            <h4>Trade AMB On Binance. The Worlds Largest Cryptocurrency Exchange</h4>
            <h3>
            Use AMB Bridge
            </h3>
        </div>
        <div class=("div_section")>
            <h1>
            Bridge AMB
            </h1>
            <h4>
            Securely Bridge AMB Between AirDAO And Compatible Blockchains
            </h4>
            <h3>
            Use AMB Bridge
            </h3>
        </div>
        </section>
    }
}

#[component]
fn Navbar() -> impl IntoView {
    view! {
        <div class=("nav_div")>
        <div>
            <h1>
            AirDAO
            </h1>
        </div>

        <ul class=("navbar")>
        <li><a href="#">Staking</a></li>
        <li><a href="#">Bridge</a></li>
        <li><a href="#">Network Explore</a></li>
        <li><a href="#">About</a></li>
        </ul>

        <button class=("button_navbar")>Connect Wallet</button>

        </div>
    }
}