// webpages/src/page_reflecting_on_a_dangerous_world.rs

use crate::template::{PAGE_TOP,PAGE_CONTENT,PAGE_BOTTOM};
use webcomponents::page_header_001::page_header_001;

pub async fn page_reflecting_on_the_world_of_danger_001_html()->String{

    let page_header = page_header_001("reflecting_on_a_dangerous_world","page_reflecting_on_a_dangerous_world");

    let css_files_names: [&str;3]=[
        "page_header_001",
        "global_styles",
        "page_articles_create",
    ];

    let css_links_markup:String = css_files_names
        .iter()
        .map(|i| format!(r#"<link rel="stylesheet" href="{}.css">"#,i))
        .collect();

    format!(
        "{}{}{}{}{}{}",
        PAGE_TOP,
        css_links_markup,
        PAGE_CONTENT,
        page_header,
        r#"
        <div id="page_authentication_screen"></div>
        <script src="page_articles_create.js"></script>


















        "#,
        PAGE_BOTTOM
    )
}
