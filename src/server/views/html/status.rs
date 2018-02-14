use hyper::Response;
use maud::{Markup, html};
use ordermap::OrderMap;

use ::engine::AnalyzeDependenciesOutcome;
use ::models::crates::{CrateName, AnalyzedDependency, AnalyzedDependencies};
use ::models::repo::{RepoSite, RepoPath};

use super::super::badge;

fn dependency_tables(crate_name: CrateName, deps: AnalyzedDependencies) -> Markup {
    html! {
        h2 class="title is-3" {
            "Crate "
            code (crate_name.as_ref())
        }

        @if deps.main.is_empty() && deps.dev.is_empty() && deps.build.is_empty() {
            p class="notification has-text-centered" "No external dependencies! 🙌"
        }

        @if !deps.main.is_empty() {
            (dependency_table("Dependencies", deps.main))
        }

        @if !deps.dev.is_empty() {
            (dependency_table("Dev dependencies", deps.dev))
        }

        @if !deps.build.is_empty() {
            (dependency_table("Build dependencies", deps.build))
        }
    }
}

fn dependency_table(title: &str, deps: OrderMap<CrateName, AnalyzedDependency>) -> Markup {
    let count_total = deps.len();
    let count_outdated = deps.iter().filter(|&(_, dep)| dep.is_outdated()).count();

    html! {
        h3 class="title is-4" (title)
        p class="subtitle is-5" {
            @if count_outdated > 0 {
                (format!(" ({} total, {} up-to-date, {} outdated)", count_total, count_total - count_outdated, count_outdated))
            } @else {
                (format!(" ({} total, all up-to-date)", count_total))
            }
        }

        table class="table is-fullwidth is-striped is-hoverable" {
            thead {
                tr {
                    th "Crate"
                    th class="has-text-right" "Required"
                    th class="has-text-right" "Latest"
                    th class="has-text-right" "Status"
                }
            }
            tbody {
                @for (name, dep) in deps {
                    tr {
                        td {
                            a href=(format!("https://crates.io/crates/{}", name.as_ref())) (name.as_ref())
                        }
                        td class="has-text-right" code (dep.required.to_string())
                        td class="has-text-right" {
                            @if let Some(ref latest) = dep.latest {
                                code (latest.to_string())
                            } @else {
                                "N/A"
                            }
                        }
                        td class="has-text-right" {
                            @if dep.is_outdated() {
                                span class="tag is-warning" "out of date"
                            } @else {
                                span class="tag is-success" "up to date"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_site_icon(repo_site: &RepoSite) -> &'static str {
    match *repo_site {
        RepoSite::Github => "fa-github",
        RepoSite::Gitlab => "fa-gitlab",
        RepoSite::Bitbucket => "fa-bitbucket",
    }
}

fn render_failure(repo_path: RepoPath) -> Markup {
    let site_icon = get_site_icon(&repo_path.site);
    html! {
        section class="hero is-light" {
            div class="hero-head" (super::render_navbar())
            div class="hero-body" {
                div class="container" {
                    h1 class="title is-1" {
                        a href=(format!("{}/{}/{}", repo_path.site.to_base_uri(), repo_path.qual.as_ref(), repo_path.name.as_ref())) {
                            i class=(format!("fa {}", site_icon)) ""
                            (format!(" {} / {}", repo_path.qual.as_ref(), repo_path.name.as_ref()))
                        }
                    }
                }
            }
        }
        section class="section" {
            div class="container" {
                div class="notification is-danger" {
                    h2 class="title is-3" "Failed to analyze repository"
                    p "The repository you requested might be structured in an uncommon way that is not yet supported."
                }
            }
        }
        (super::render_footer(None))
    }
}

fn render_success(analysis_outcome: AnalyzeDependenciesOutcome, repo_path: RepoPath) -> Markup {
    let self_path = format!("repo/{}/{}/{}", repo_path.site.as_ref(), repo_path.qual.as_ref(), repo_path.name.as_ref());
    let status_base_url = format!("{}/{}", &super::SELF_BASE_URL as &str, self_path);
    let site_icon = get_site_icon(&repo_path.site);

    let status_data_uri = badge::badge(Some(&analysis_outcome)).to_svg_data_uri();

    let hero_class = if analysis_outcome.any_outdated() {
        "is-warning"
    } else {
        "is-success"
    };

    html! {
        section class=(format!("hero {}", hero_class)) {
            div class="hero-head" (super::render_navbar())
            div class="hero-body" {
                div class="container" {
                    h1 class="title is-1" {
                        a href=(format!("{}/{}/{}", repo_path.site.to_base_uri(), repo_path.qual.as_ref(), repo_path.name.as_ref())) {
                            i class=(format!("fa {}", site_icon)) ""
                            (format!(" {} / {}", repo_path.qual.as_ref(), repo_path.name.as_ref()))
                        }
                    }

                    img src=(status_data_uri);
                }
            }
            div class="hero-footer" {
                div class="container" {
                    div class="tabs" {
                        ul {
                            li class="is-active" {
                                a href="#markdown" {
                                    "Markdown"
                                }
                            }
                            li {
                                a href="#asciidoc" {
                                    "Asciidoc"
                                }
                            }
                        }
                    }
                }
                div class ="container" {
                    div class="sources" {
                        div id="markdown" class="is-active" {
                            pre class="is-size-7" {
                                (format!("[![dependency status]({}/status.svg)]({})\n", status_base_url, status_base_url))
                            }
                        }
                        div id="asciidoc" class="is-hidden" {
                            pre class="is-size-7" {
                                (format!(r#"image::{}/status.svg[link="{}",alt="dependency status"\n"#, status_base_url, status_base_url))
                            }
                        }
                    }
                }
            }
        }
        section class="section" {
            div class="container" {
                @for (crate_name, deps) in analysis_outcome.crates {
                    (dependency_tables(crate_name, deps))
                }
            }
        }
        (super::render_footer(Some(analysis_outcome.duration)))
    }
}

pub fn render(analysis_outcome: Option<AnalyzeDependenciesOutcome>, repo_path: RepoPath) -> Response {
    let title = format!("{} / {}", repo_path.qual.as_ref(), repo_path.name.as_ref());

    if let Some(outcome) = analysis_outcome {
        super::render_html(&title, render_success(outcome, repo_path))
    } else {
        super::render_html(&title, render_failure(repo_path))
    }
}
