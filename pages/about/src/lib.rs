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
                <GradientTitle>{"聊聊设计"}</GradientTitle>
                <p>
                    {"如果你有注意到之前的 zzhack 版本的设计，你会发现这次的设计是有很大程度的风格上的变化的，从的内容板块到卡片式驱动的风格，从严肃到活泼，至少从第一感官上来看，是有很大变化的。"}
                </p>
                <img class="illustrate" src="/images/about_compare.svg" />
                <p>
                    {"第五版（右）对比第三版看起来卡片式的风格会让我觉得舒服很多，但不幸的是，我对自己的设计并没有很满意，无论从设计方法还是设计思想来看，都太过于青涩，这也是我第一次尝试自己思考和画设计稿。你可能会有一种感觉，到现在的 UI 整体看上去总觉得哪里不太搭，但又说不出来，仔细看细节好像也很合理… 这种感觉是对的，存在这样问题的根本原因是 zzhack 的 UI 并没有对应合理严谨的设计语言来进行支撑，它和现在很多 UI 库包括很多网站一样，存在这样的问题。"}
                </p>
                <p>{"根本上，它们都是拼凑的，而不是设计的。"}</p>
                <img class="illustrate" src="/images/about_design_lang.svg" />
                <p>
                    {"事实上，UI 是面向用户的第一层，是直接影响用户使用体验的东西，大多数用户不会遇到有计算/渲染瓶颈的场景，那事实上一大部分性能就过剩了，客户端体验的决定权就会有很大一部分在 UI，所以设计在产品设计中是很重要的一节。那对于我来说，zzhack 之所以没有采用现在市面上流行成熟的设计语言作为基础吃撑来进行设计，是因为 zzhack 不是一个商业化的产品，所以没有必要，它具有强烈个性的原因之一正是因为它没有设计语言来做支撑。"}
                </p>
            </section>
            <section>
                <GradientTitle>{"关于我"}</GradientTitle>
                <p>
                    {"我叫 Mist，一名软件工程师。"}
                </p>
            </section>
        </div>
    }
}
