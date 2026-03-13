use ssg_whiz::summaries::{Category, PageSummary, Summary};

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "Updates".to_string(),
            pages: vec![PageSummary {
                date: "2026-03-09",
                title: "Let's talk about Agentic Sandboxes",
                description: "Why safe execution boundaries are central to useful coding agents.",
                folder: "blog/agentic-sandboxes",
                markdown: include_str!("../content/blog/agentic-sandboxes/index.md"),
                image: Some("/blog/agentic-sandboxes/agentic-ai-sandboxing.png"),
                author: None,
                author_image: None,
            }],
        }],
    }
}
