use stylist::style;
use ui::contact::ContactType;
use ui::gradient_title::GradientTitle;
use ui::link::Link;
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
                <GradientTitle>{"zzhack 的诞生"}</GradientTitle>
                <div>
                    <p>
                        {"嗨！欢迎来到我的应用 zzhack 😎，这是一个兴趣使然的项目，zzhack 被设计为一个注重信息展示的应用，它是序列化和沉淀我思想的地方。"}
                    </p>
                    <p>
                       {"如你所见的 zzhack 已是第五个大版本，它已经经过了两次大规模的重构以及 5 次重新设计，最后回归纯真，专注信息展示。"}
                    </p>
                </div>
                <img class="illustrate" src="/images/about_zzhack.svg" />
                <p>
                    {"这么看下来 zzhack 的确没有什么让人惊讶的亮点，没有额外的用户交互，没有炫酷的交互动画，看上去只是一个平静的展示内容的 web 应用，但是它的确适合作为一个单纯的内容输出的站点，而不被逐渐社交化。"}
                </p>
                <p>
                    {"zzhack 是一个纯静态的应用，并且开源内容到代码的所有，如果你对它的技术实现感兴趣可以在 "}
                    <Link out_href={ContactType::GitHub.into_lnk()}>{"这里"}</Link>
                    {" 找到它。"}
                </p>
            </section>
            <section>
                <GradientTitle>{"关于我"}</GradientTitle>
                <p>
                    {"我叫 Mist，一名前端工程师。"}
                </p>
            </section>
        </div>
    }
}
