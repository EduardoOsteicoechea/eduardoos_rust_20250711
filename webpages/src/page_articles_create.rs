// rust-http-server-80/webpages/src/page_articles_create.rs

use crate::template::page_generator;

pub async fn page_articles_create_html()->String{

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
    <div 
    id="authentication_screen"
    class="authentication_screen"    
    >

        <div
        class="authentication_labels_container"
        >
            <label 
            for="authentication_username_input"
            id="authentication_password_label"
            class="authentication_screen_label authentication_screen_control"
            >
                Username
            </label>
        
            <label 
            for="authentication_password_input"
            id="authentication_password_label"
            class="authentication_screen_label authentication_screen_control"
            >
                Password
            </label>
        </div>

        <div
        class="authentication_inputs_container"
        >
            <input
            id="authentication_username_input"
            class="authentication_screen_input authentication_screen_control"
            type="text"
            >

            <input 
            id="authentication_password_input" 
            class="authentication_screen_input authentication_screen_control"
            type="password"
            >

        </div>

        <button
        id="authentication_authenticate_button"
        class="authentication_screen_autentication_button authentication_screen_control"
        type="button"
        >
            Sign In
        </button>
    </div>    
    "#;

    page_generator(
        page_title,
        css_files_names,
        js_head_files_names,
        js_bottom_files_names,
        page_main_content
    ).await
}
