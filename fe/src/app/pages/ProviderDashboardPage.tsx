import { createMemo, createResource, createSignal, For, Show } from "solid-js";

import {
  createProviderBlogPost,
  getProviderDashboard,
  upsertProviderMarketplaceProfile
} from "../../api/marketplaceApi";
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
  const [editorResetToken, setEditorResetToken] = createSignal(0);
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
      status: "published"
    });
    setNotice(result.ok ? "Blog post saved." : result.error.message);
    if (result.ok) {
      setRefreshTick((value) => value + 1);
      setBlogTitle("");
      setBlogExcerpt("");
      setBlogBody("");
      setEditorResetToken((value) => value + 1);
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
                  <strong>{post.title}</strong>
                  <span class="marketplace-chip">{post.moderation_status}</span>
                </div>
              )}
            </For>
          </div>
        </section>
      </Show>
    </section>
  );
}
