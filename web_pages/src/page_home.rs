use crate::template::{PAGE_TOP, PAGE_CONTENT, PAGE_BOTTOM};

pub async fn retrieve_page_html_string() -> String {

    format!(
            "{}{}{}{}",
            PAGE_TOP,
            PAGE_CONTENT,
            r#"
            <h1>Thanks Lord</h1>
            <h2>Thnaks Jesus</h2>
            <script src="display_todo_tasks.js" defer></script>
            "#,
            PAGE_BOTTOM
        )

}
