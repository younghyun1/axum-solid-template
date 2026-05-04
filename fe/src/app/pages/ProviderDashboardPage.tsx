import { createMemo, createResource, createSignal, For, Show } from "solid-js";

import {
  createProviderBlogPost,
  getProviderDashboard,
  updateProviderBlogPost,
  upsertProviderMarketplaceProfile
} from "../../api/marketplaceApi";
import type { BlogPostStatus, ProviderBlogPostResponse } from "../../api/marketplaceTypes";
import type { MeResponse } from "../../api/types";
import { resultData } from "../helpers";
import { MarkdownEditor } from "../shared/MarkdownEditor";

interface ProviderDashboardPageProps {
  readonly profile: MeResponse | null;
}

export function ProviderDashboardPage(props: ProviderDashboardPageProps) {
  const canManage = createMemo(() => {
    const role = props.profile?.claims.role_type;
    return role === "service_provider" || role === "admin";
  });
  const [refreshTick, setRefreshTick] = createSignal(0);
  const [dashboardResult] = createResource(refreshTick, getProviderDashboard);
  const dashboard = createMemo(() => resultData(dashboardResult()));
  const [displayName, setDisplayName] = createSignal(props.profile?.user_info.user_name ?? "");
  const [slug, setSlug] = createSignal("");
  const [headline, setHeadline] = createSignal("");
  const [blogTitle, setBlogTitle] = createSignal("");
  const [blogExcerpt, setBlogExcerpt] = createSignal("");
  const [blogBody, setBlogBody] = createSignal("");
  const [blogStatus, setBlogStatus] = createSignal<BlogPostStatus>("published");
  const [editorResetToken, setEditorResetToken] = createSignal(0);
  const [editingPostId, setEditingPostId] = createSignal<string | null>(null);
  const [editTitle, setEditTitle] = createSignal("");
  const [editExcerpt, setEditExcerpt] = createSignal("");
  const [editBody, setEditBody] = createSignal("");
  const [editStatus, setEditStatus] = createSignal<BlogPostStatus>("draft");
  const [editResetToken, setEditResetToken] = createSignal(0);
  const [notice, setNotice] = createSignal("");

  const saveProfile = async () => {
    const result = await upsertProviderMarketplaceProfile({
      slug: slug().trim() || null,
      display_name: displayName().trim(),
      headline: headline().trim() || null,
      bio: null,
      service_area: null,
      status: "published"
    });
    setNotice(result.ok ? "Provider profile saved." : result.error.message);
    if (result.ok) {
      setRefreshTick((value) => value + 1);
    }
  };

  const publishPost = async () => {
    const result = await createProviderBlogPost({
      slug: null,
      title: blogTitle().trim(),
      excerpt: blogExcerpt().trim() || null,
      body: blogBody().trim(),
      status: blogStatus()
    });
    setNotice(result.ok ? "Blog post saved." : result.error.message);
    if (result.ok) {
      setRefreshTick((value) => value + 1);
      setBlogTitle("");
      setBlogExcerpt("");
      setBlogBody("");
      setBlogStatus("published");
      setEditorResetToken((value) => value + 1);
    }
  };

  const startEdit = (post: ProviderBlogPostResponse) => {
    setEditingPostId(post.provider_blog_post_id);
    setEditTitle(post.title);
    setEditExcerpt(post.excerpt ?? "");
    setEditBody(post.body ?? "");
    setEditStatus(post.status);
    setEditResetToken((value) => value + 1);
  };

  const cancelEdit = () => {
    setEditingPostId(null);
    setEditTitle("");
    setEditExcerpt("");
    setEditBody("");
    setEditStatus("draft");
    setEditResetToken((value) => value + 1);
  };

  const saveEditedPost = async () => {
    const postId = editingPostId();
    if (postId === null) {
      return;
    }
    const result = await updateProviderBlogPost(postId, {
      slug: null,
      title: editTitle().trim(),
      excerpt: editExcerpt().trim() || null,
      body: editBody().trim(),
      status: editStatus()
    });
    setNotice(result.ok ? "Blog post updated and sent to moderation." : result.error.message);
    if (result.ok) {
      setRefreshTick((value) => value + 1);
      cancelEdit();
    }
  };

  return (
    <section class="marketplace-layout">
      <Show when={canManage()} fallback={<p class="marketplace-empty">Provider role required.</p>}>
        <div class="marketplace-heading">
          <p class="eyebrow">Provider dashboard</p>
          <h1>{dashboard()?.profile.display_name ?? "Provider profile"}</h1>
        </div>

        <div class="marketplace-columns">
          <section class="marketplace-panel">
            <h2>Profile</h2>
            <div class="flow-form">
              <input value={displayName()} onInput={(event) => setDisplayName(event.currentTarget.value)} />
              <input placeholder="public-slug" value={slug()} onInput={(event) => setSlug(event.currentTarget.value)} />
              <input placeholder="Headline" value={headline()} onInput={(event) => setHeadline(event.currentTarget.value)} />
              <button class="primary-button" type="button" onClick={saveProfile}>Save profile</button>
            </div>
          </section>

          <section class="marketplace-panel">
            <h2>Blog</h2>
            <div class="flow-form">
              <input placeholder="Post title" value={blogTitle()} onInput={(event) => setBlogTitle(event.currentTarget.value)} />
              <input placeholder="Short excerpt" value={blogExcerpt()} onInput={(event) => setBlogExcerpt(event.currentTarget.value)} />
              <select
                value={blogStatus()}
                onInput={(event) => setBlogStatus(event.currentTarget.value as BlogPostStatus)}
              >
                <option value="published">Publish</option>
                <option value="draft">Save draft</option>
              </select>
              <MarkdownEditor
                label="Post body"
                value={blogBody()}
                resetToken={editorResetToken()}
                onChange={setBlogBody}
              />
              <button class="secondary-button" type="button" onClick={publishPost}>Publish post</button>
            </div>
          </section>
        </div>

        <Show when={notice()}>
          {(text) => <p class="field-note">{text()}</p>}
        </Show>

        <section class="marketplace-panel">
          <h2>Existing posts</h2>
          <div class="marketplace-list">
            <For each={dashboard()?.blog_posts ?? []}>
              {(post) => (
                <div class="marketplace-row">
                  <div>
                    <strong>{post.title}</strong>
                    <p>{post.status}</p>
                  </div>
                  <div class="marketplace-row__actions">
                    <span class="marketplace-chip">{post.moderation_status}</span>
                    <button class="secondary-button" type="button" onClick={() => startEdit(post)}>
                      Edit
                    </button>
                  </div>
                </div>
              )}
            </For>
          </div>
        </section>

        <Show when={editingPostId() !== null}>
          <section class="marketplace-panel">
            <h2>Edit post</h2>
            <div class="flow-form">
              <input value={editTitle()} onInput={(event) => setEditTitle(event.currentTarget.value)} />
              <input value={editExcerpt()} onInput={(event) => setEditExcerpt(event.currentTarget.value)} />
              <select
                value={editStatus()}
                onInput={(event) => setEditStatus(event.currentTarget.value as BlogPostStatus)}
              >
                <option value="published">Publish</option>
                <option value="draft">Draft</option>
                <option value="archived">Archive</option>
              </select>
              <MarkdownEditor
                label="Post body"
                value={editBody()}
                resetToken={editResetToken()}
                onChange={setEditBody}
              />
              <div class="marketplace-action-row">
                <button class="primary-button" type="button" onClick={saveEditedPost}>
                  Save changes
                </button>
                <button class="secondary-button" type="button" onClick={cancelEdit}>
                  Cancel
                </button>
              </div>
            </div>
          </section>
        </Show>
      </Show>
    </section>
  );
}
