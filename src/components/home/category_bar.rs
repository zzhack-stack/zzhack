use yew::prelude::*;
use css_in_rust::style::Style;
use material_yew:: {
    MatTab,
    MatTabBar
};

#[derive(Properties, Clone)]
pub struct CategoryBarProps {
    pub categories: Vec<&'static str>,
    pub text: &'static str
}


pub struct CategoryBar {
    props: CategoryBarProps,
    link: ComponentLink<CategoryBar>,
    style: Style
}

impl Component for CategoryBar {
    type Message = ();
    type Properties = CategoryBarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create("CategoryBar", r#"
            width: 100%;
            height: 118px;
            background: var(--undercover-color);

            .text {
                font-family: 'Roboto';
                font-size: 16px;
                height: 60px;
                display: flex;
                align-items: center;
                padding: 10px 24px 0 24px;
                color: #636e72;
            }

            .tabs {
                height: 48px;
                display: flex;
            }
        "#).unwrap();

        Self {
            props,
            link,
            style
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <div class="text">
                    {self.props.text}
                </div>
                <div class="tabs">
                    <MatTabBar>
                        {for self.props.categories.iter().map(|name| html!{
                            <MatTab label=name.clone() is_min_width_indicator=true min_width=true is_fading_indicator=true />
                        })}
                    </MatTabBar>
                </div>
            </div>
        }
    }
}