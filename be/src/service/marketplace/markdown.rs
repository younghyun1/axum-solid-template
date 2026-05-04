use ammonia::Builder;
use comrak::{Options, markdown_to_html};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedMarkdown {
    pub html: String,
}

pub fn render_markdown(markdown: &str) -> RenderedMarkdown {
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.render.r#unsafe = false;

    let html = markdown_to_html(markdown, &options);
    let sanitized = Builder::default().clean(&html).to_string();

    RenderedMarkdown { html: sanitized }
}

#[cfg(test)]
mod tests {
    use super::render_markdown;

    #[test]
    fn render_markdown_keeps_gfm_tables() {
        let rendered = render_markdown("| A | B |\n| - | - |\n| 1 | 2 |");

        assert!(rendered.html.contains("<table>"));
        assert!(rendered.html.contains("<td>1</td>"));
    }

    #[test]
    fn render_markdown_removes_script_content() {
        let rendered = render_markdown("# Safe\n\n<script>alert('x')</script>");

        assert!(rendered.html.contains("<h1>Safe</h1>"));
        assert!(!rendered.html.contains("<script>"));
        assert!(!rendered.html.contains("alert"));
    }

    #[test]
    fn render_markdown_removes_event_attributes() {
        let rendered = render_markdown("<img src=\"x\" onerror=\"alert(1)\">");

        assert!(!rendered.html.contains("onerror"));
        assert!(!rendered.html.contains("alert"));
    }
}
