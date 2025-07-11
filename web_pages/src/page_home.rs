use crate::template::{PAGE_TOP, PAGE_CONTENT, PAGE_BOTTOM};
use webcomponents::page_header_001::page_header_001;


pub async fn retrieve_page_html_string() -> String {

    let page_header = page_header_001("","home");

    let css_files_names: [&str;2] = [
        "page_header_001",
        "global_styles",
    ];

    let css_links_markup: String = css_files_names
        .iter()
        .map(|item| format!(r#"<link rel="stylesheet" href="{}.css">"#, item))
        .collect();


    format!(
            "{}{}{}{}{}{}",
            PAGE_TOP,
            css_links_markup,
            PAGE_CONTENT,
            page_header,
            r#"
            <h1>Thanks Lord</h1>
            <h2>Thnaks Jesus</h2>
            <script src="display_todo_tasks.js" defer></script>
            "#,
            PAGE_BOTTOM
        )

}
