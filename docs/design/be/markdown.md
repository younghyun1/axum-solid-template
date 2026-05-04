# Backend Markdown Design

Blog content stores markdown source as canonical data. Public APIs may expose sanitized rendered HTML where useful, but the source remains the editable representation.

Rendering uses a CommonMark/GFM-compatible parser and an HTML sanitizer. Raw HTML in markdown is not trusted. Sanitization must remove scripts, event attributes, unsafe protocols, and unexpected tags.

Rendered HTML can be cached and indexed as plain text. Rendering failures return structured API errors and log the content owner, content id, and parser error where available.

Provider-created posts remain moderation-controlled. Public reads only return published posts with approved moderation state.
