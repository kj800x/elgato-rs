use maud::{DOCTYPE, Markup, html};

pub fn layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " | Key Light" }
                link rel="stylesheet" href="/static/style.css";
                script src="/static/htmx.min.js" {}
                script {
                    "htmx.config.responseHandling = [\
                        {code:'204', swap: false},\
                        {code:'[23]..', swap: true},\
                        {code:'[45]..', swap: true, error: true}\
                    ];"
                }
            }
            body {
                header .app-header {
                    h1 .logo { "Key Light" }
                }
                main .content {
                    (content)
                }
            }
        }
    }
}
