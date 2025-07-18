// rust-http-server-80/webpages/src/layout.rs

use webcomponents::page_header_001::page_header_001;

pub const PAGE_TOP: &'static str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width,initial-scale=1.0">
"#;

pub const PAGE_CONTENT: &'static str = r#"
    </head>
    <body>
"#;

pub const PAGE_BOTTOM: &'static str = r#"
    </body>
    </html>
"#;

fn link_or_script_tag_generator
(
    is_css:bool,
    root_path:&str,
    files_names:Vec<&str>,
)
-> String 
{
    
    let tag:&str = if is_css {"link"} else {"script"};

    let closing:String = if !is_css 
    {
        format!(r#"</{}>"#,tag)
    }
    else
    {
        "".to_string()
    };

    let mut result = String::new();
    for file_name in files_names 
    {
        let attributes = if is_css {
            format!(r#" rel="stylesheet" href="{}.css""#,file_name)
        }
        else
        {
            format!(r#" src="{}.js""#,file_name)
        };

        result.push_str(format!(r#"<{}{}>{}"#,tag,attributes,closing).as_str());
    };

    result
}

pub async fn page_generator
(
    root_path :&str,
    page_title :&str,
    css_files_names :Vec<&str>,
    js_head_files_names :Vec<&str>,
    js_bottom_files_names :Vec<&str>,
    page_main_content :&str,
)
-> String 
{
    let page_header:String = page_header_001(root_path,page_title);

    let page_title_tag:String = format!(r#"<title>{}</title>"#,page_title);

    let css_tipical_tags:String = link_or_script_tag_generator(
        true,
        root_path,
        vec!["page_header_001","global_styles"],
    );

    let js_head_tipical_tags:String = link_or_script_tag_generator(
        false,
        root_path,
        vec!["global"],
    );

    let js_bottom_tipical_tags:String = String::new();

    let css_current_page_tags:String = if css_files_names.len() == 0 {
        String::new()
    } else {
        link_or_script_tag_generator(true,root_path,css_files_names)
    };    

    let js_head_current_page_tags:String = if js_head_files_names.len() == 0 {
        String::new()
    } else {
        link_or_script_tag_generator(false,root_path,js_head_files_names)
    };

    let js_bottom_current_page_tags:String = if js_bottom_files_names.len() == 0 {
        String::new()
    } else {
        link_or_script_tag_generator(false,root_path,js_bottom_files_names)
    };

    format!(
        r#"
        {PAGE_TOP}
        {PAGE_TITLE_TAG}

        {CSS_TIPICAL_TAGS}
        {CSS_CURRENT_PAGE_TAGS}

        {JS_HEAD_TIPICAL_TAGS}
        {JS_HEAD_CURRENT_PAGE_TAGS}

        {PAGE_CONTENT}
        {PAGE_HEADER}
        {PAGE_MAIN_CONTENT}

        {JS_BOTTOM_TIPICAL_TAGS}
        {JS_BOTTOM_CURRENT_PAGE_TAGS}

        {PAGE_BOTTOM}
        "#,
        PAGE_TOP=PAGE_TOP,
        PAGE_TITLE_TAG=page_title_tag,

        CSS_TIPICAL_TAGS = css_tipical_tags,
        CSS_CURRENT_PAGE_TAGS = css_current_page_tags,

        JS_HEAD_TIPICAL_TAGS = js_head_tipical_tags,
        JS_HEAD_CURRENT_PAGE_TAGS = js_head_current_page_tags,

        PAGE_CONTENT=PAGE_CONTENT,
        PAGE_HEADER=page_header,
        PAGE_MAIN_CONTENT=page_main_content,

        JS_BOTTOM_TIPICAL_TAGS = js_bottom_tipical_tags,
        JS_BOTTOM_CURRENT_PAGE_TAGS = js_bottom_current_page_tags,

        PAGE_BOTTOM=PAGE_BOTTOM
    )
}
