use leptos::*;

fn main() {
    leptos::mount_to_body(|| {
        view! {
           <Navbar />
           <Header/>
           <Section />
           <SecondSection />
           <StakeSection />
           <ValidatorSection />
        }
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
            <a href="#">
            Use AMB Bridge
            </a>
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
            <a href="#">
            Use AMB Bridge
            </a>
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

#[component]
fn SecondSection() -> impl IntoView {
    view! {
    <section class=("second_section")>
        <h1 class=("big_title")>Firepot Swap</h1>
        <div class=("container_section")>
        <h3>Buy And Trade AMB And Other Crypto With FirepotSwap.
        AirDAOs Native Decentralized Exchange.
        </h3>
        <div class=("container_buttons")>
            <button class=("button_style_2")>Use Firepotswap</button>
            <button class=("button_style_2")>Learn More</button>
        </div>
        </div>
        <div class="box_section">
        <div>
        <h1 class=("section_headers")>MEXC</h1>
        <p>MEXC Provides A Platform For Users To Trade A Vast Array Of Cryptocurrencies</p>
        </div>
        <div>
        <h1 class=("section_headers")>Binance</h1>
        <p>Binance Has Its Own Native Utility Token Called WBTC.
        Holders Of WBTC May Enjoy Certain</p>
        </div>
        <div>
        <h1 class=("section_headers")>KuCoin</h1>
        <p>KuCoin Provides A Platform For Trading A Diverse Selection Of Cryptocurrencies</p>
        </div>
        <div>
        <h1 class=("section_headers")>WhiteBIT</h1>
        <p>WhiteBit Has Its Own Native Utility Token Called WBTC.
        Holders Of WBTC May Enjoy Certain</p>
        </div>
        </div>
    </section>
    }
}

#[component]
fn StakeSection() -> impl IntoView {
    view! {
    <section>
        <div class=("stake_row")>
            <div class=("title_div")>
            <h1 class=("big_title")>Stake AMB</h1>
            </div>
            <div class=("element_wrapper")>
            <div class=("button_flex")>
            <p>Buy An Trade AMB And Other Crypto With Firepot Swap AirDAOs Native Decentralized Exchange</p>
            <button class=("button_style_3")>Start Earning</button>
            </div>
            <div class=("section_wrapper")>
            <div>
            <h3>Starting At 1000 AMB</h3>
            <p>Blockchain Networks Offer Rewards To Users Who Lock Up Their Tokens In A Process Called Staking</p>
            </div>
            <div>
            <h3>No Technical Expertise Needed</h3>
            <p>Technical Expertise Typically Begins With A Strong Foundation Of Knowledge In A Particular Field</p>
            </div>
            </div>
            <div class=("section_wrapper_2")>
            <div>
            <h3>Unstake At Any Time</h3>
            <p>Taking Involves Locking Up Your Cryptocurrency Assets. Typically In A Smart Contract As Collateral To Support the Ope...</p>
            </div>
            <div>
            <h3>Receive Rewards Every 6 Hours</h3>
            <p>No Proof Of Work Blockchain Networks Like Bitcoin Miners Use Computational Power to Solve Complex Mathematical Problems An...</p>
            </div>
            </div>
            </div>

        </div>
    </section>
    }
}

#[component]

fn ValidatorSection() -> impl IntoView {
    view! {
        <section>
        <div class=("validator-div")>
        <h1>Become A Validator</h1>
        <p>
        Buy and Trade AMB And Other Crypto With Firepot Swap AirDAOs Native Decentralized Exchange
        </p>
        </div>
        <div class=("validator-div-2")>
        <div>
        <h1 class=("secondary-title")>15,000+</h1>
        <h2>Total AMB Holders</h2>
        </div>
        <div>
        <h1 class=("secondary-title")>500+</h1>
        <h2>Total AMB Holders</h2>
        </div>
        </div>
        <div class=("validator-div-3")>
        <div>
        <h3>Roadmap</h3>
        <p>"Discover What's Coming To AirDao In 2023 And Beyond"</p>
        </div>
        <div>
        <h3>Ambassador Program</h3>
        <p>"Discover What's Coming To AirDao In 2023 And Beyond"</p>
        </div>
        <div>
        <h3>AirDAO Network</h3>
        <p>"Discover What's Coming To AirDao In 2023 And Beyond"</p>
        </div>
        </div>
        </section>
    }
}
