use yew::prelude::*;

pub fn render_with_insert_node(nodes: &Vec<Html>, node: &Html) -> Html {
    let render_nodes = nodes[1..].iter();
    let rendered_nodes = render_nodes
        .map(|render_node| {
            html! {
                <>
                    {render_node.clone()}
                    {node.clone()}
                </>
            }
        })
        .collect::<Html>();

    html! {
        <>
            {rendered_nodes}
            {nodes.get(0).unwrap().clone()}
        </>
    }
}
