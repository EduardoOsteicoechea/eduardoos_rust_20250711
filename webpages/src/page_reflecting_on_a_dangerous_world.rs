// webpages/src/page_reflecting_on_a_dangerous_world.rs

use crate::template::{PAGE_TOP,PAGE_CONTENT,PAGE_BOTTOM};
use webcomponents::page_header_001::page_header_001;

pub async fn page_reflecting_on_a_dangerous_world()->String{

    let page_header = page_header_001("reflecting_on_a_dangerous_world","page_reflecting_on_a_dangerous_world");

    let css_files_names: [&str;2]=[
        "page_header_001",
        "global_styles.css",
        "article_styles_001.css",
    ];

    let css_links_markup:String = css_files_names
        .iter()
        .map(|i| format!(r#"<link rel="stylesheet" href="{}.css">"#,i))
        .collect();

    format!(
        "{}{}{}{}{}{}",
        PAGE_TOP,
        css_links_markup,
        page_header,
        r#"

this is the page


















        "#,
        PAGE_BOTTOM
    )
}
