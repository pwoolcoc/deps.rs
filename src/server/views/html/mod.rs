use std::env;
use std::time::Duration;

use hyper::Response;
use hyper::header::ContentType;
use maud::{Markup, Render, html};

pub mod index;
pub mod error;
pub mod status;

lazy_static! {
    static ref SELF_BASE_URL: String = {
        env::var("BASE_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string())
    };

    static ref GAUGES_SITE_ID: Option<String> = {
        env::var("GAUGES_SITE_ID").ok().map(|s| s.to_string())
    };
}

fn render_html<B: Render>(title: &str, body: B) -> Response {
    let rendered = html! {
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title (format!("{} - Deps.rs", title))
                link rel="icon" type="image/png" href="/static/favicon.png";
                link rel="stylesheet" type="text/css" href="/static/style.css";
                link rel="stylesheet" type="text/css" href="https://fonts.googleapis.com/css?family=Fira+Sans:400,500,600";
                link rel="stylesheet" type="text/css" href="https://fonts.googleapis.com/css?family=Source+Code+Pro";
                link rel="stylesheet" type="text/css" href="https://maxcdn.bootstrapcdn.com/font-awesome/4.7.0/css/font-awesome.min.css";
            }
            body {
                (body)
                script type="text/javascript" src="/static/app.js"
                @if let Some(site_id) = GAUGES_SITE_ID.as_ref() {
                    script type="text/javascript" {
                        (format!("var _gauges = _gauges || [];
(function() {{
    var t   = document.createElement('script');
    t.type  = 'text/javascript';
    t.async = true;
    t.id    = 'gauges-tracker';
    t.setAttribute('data-site-id', '{}');
    t.setAttribute('data-track-path', 'https://track.gaug.es/track.gif');
    t.src = 'https://d2fuc4clr7gvcn.cloudfront.net/track.js';
    var s = document.getElementsByTagName('script')[0];
    s.parentNode.insertBefore(t, s);
}})();", site_id))
                    }
                }
            }
        }
    };

    Response::new()
        .with_header(ContentType::html())
        .with_body(rendered.0)
}

fn render_navbar() -> Markup {
    html! {
        header class="navbar" {
            div class="container" {
                div class="navbar-brand" {
                    a class="navbar-item is-dark" href=(SELF_BASE_URL) {
                        h1 class="title is-3" "Deps.rs"
                    }
                }
            }
        }
    }
}

fn render_footer(duration: Option<Duration>) -> Markup {
    let duration_millis = duration.map(|d| d.as_secs() * 1000 + (d.subsec_nanos() / 1000 / 1000) as u64);

    html! {
        footer class="footer" {
            div class="container" {
                div class="content has-text-centered" {
                    p {
                        strong "Deps.rs"
                        " is a service for the Rust community. It is open source on "
                        a href="https://github.com/srijs/deps.rs" "GitHub"
                        "."
                    }
                    p {
                        "Please report any issues on the "
                        a href="https://github.com/srijs/deps.rs/issues" "issue tracker"
                        "."
                    }
                    @if let Some(millis) = duration_millis {
                        p class="has-text-grey is-size-7" (format!("(rendered in {} ms)", millis))
                    }
                }
            }
        }
    }
}
