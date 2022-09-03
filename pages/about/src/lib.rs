use stylist::style;
use ui::gradient_title::GradientTitle;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    let style = style!(
        r"
        padding: 50px 0;

        a {
            color: var(--blue);
            font-size: 16px;
        }

        .illustrate {
            margin: 20px 0;
            border-radius: 20px;
        }

        @media (max-width: 600px) {
            .illustrate {
                width: 100%;
            }
        }
    "
    )
    .unwrap();

    html! {
        <div class={style}>
            <section>
                <img class="illustrate" src="/images/anand-pfp.svg" />
                <div>
                    <p>
                        {"I have a B.S. in Computer Science and a B.A. in Anthropology from Washington University in St. Louis."}
                        <ul>
                            <li>
                                {"I enjoy building software solutions for humans, to be used by humans."}
                            </li>
                        </ul>
                    </p>
                    <p>
                        {"I've got quite a few interests, and can do some stuff:"}
                        <ul>
                            <li>
                                {"Albeit I beep boop 🧑‍💻 now, I like to ✒️ English too."}
                            </li>
                            <li>
                                {"While I primarily use a Mac, I grew up using Windows."}
                            </li>
                            <ul>
                                <li>
                                    {"With some spare time, I like to 🧑‍🔬 with Linux (OpenBSD, Arch, WSL)."}
                                </li>
                                <li>
                                    {"I'm also a big fan of Raspberry Pi's (not the edible ones), and like tinkering with microcontrollers."}
                                </li>
                            </ul>
                            <li>
                                {"While I ❤️ Vim/Vi, (sorry Emacs gang), I'm a huge fan of VSCode/IntelliJ products."}
                            </li>
                            <ul>
                                <li>
                                    {"I like Bash and the command line, and am an unashamed Unix philosophy fanboy."}
                                </li>
                            </ul>
                            <li>
                                {"I still use IRC sometimes 🤷."}
                            </li>
                            <li>
                                {"I'm a big fan of Rust 🦀, GoLang, and TypeScript."}
                            </li>
                            <ul>
                                <li>
                                    {"I like C, but try to avoid using it, as I'm not a wizard 🧙."}
                                </li>
                            </ul>
                            <li>
                                {"I read a lot of books and research papers, and I'm constantly searching for new things to learn."}
                            </li>
                        </ul>
                    </p>
                </div>
                <p>
                    {"Outside of beep booping, I enjoy:"}
                        <ul>
                            <li>
                                {"Snowboarding 🏂"}
                            </li>
                            <li>
                                {"Tennis 🎾"}
                            </li>
                            <li>
                                {"Hiking 🥾"}
                            </li>
                            <li>
                                {"(And of course) 📚 Reading 🤓"}
                            </li>
                        </ul>
                </p>
            </section>
            <section>
                <GradientTitle>{"Anand"}</GradientTitle>
            </section>
        </div>
    }
}
