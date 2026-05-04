# Frontend Markdown Editor Design

Marketplace blog authoring uses a rich markdown editor with markdown as the persisted source.

The editor shell provides:

- title, slug, excerpt, status, and hero image controls
- rich markdown editing with headings, lists, links, tables, code blocks, and quotes
- preview/source affordances
- save, publish, archive, and moderation-aware disabled states
- API error and validation display near the affected control

The editor component is shared by provider and admin blog workflows. Page components own routing and permissions; the editor owns draft state and serialization.

Styles live under `fe/src/styles` and use shared editor classes. The editor should feel like a work tool, not a marketing page.
