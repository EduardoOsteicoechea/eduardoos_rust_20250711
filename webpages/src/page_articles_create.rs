// rust-http-server-80/webpages/src/page_articles_create.rs

use crate::template::page_generator;

pub async fn page_articles_create_html()->String{

    let root_folder:&str = "../../";
    let page_title:&str = "Articles Creator";
    let page_identifier_string:&str = "page_articles_create";
    let css_files_names:Vec<&str> = vec![
        &page_identifier_string,
    ];
    let js_head_files_names:Vec<&str> = vec![];
    let js_bottom_files_names:Vec<&str> = vec![
        &page_identifier_string,
    ];

    let page_main_content:&str = r#"
    <div id="page_authentication_screen">
	<input id="page_authentication_key" type="text">
	<button id="page_authentication_button">Authenticate</button>
    </div>    
    "#;

    page_generator(
        root_folder,
        page_title,
        css_files_names,
        js_head_files_names,
        js_bottom_files_names,
        page_main_content
    ).await
}
