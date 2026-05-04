import { createMemo, createResource, For, Show } from "solid-js";

import { getProviderDetail } from "../../api/marketplaceApi";
import type { ImageResponse, ProviderBlogPostResponse } from "../../api/marketplaceTypes";
import { resultData } from "../helpers";

interface ProviderDetailPageProps {
  readonly slug: string | null;
  readonly onBack: () => void;
  readonly onOpenPayments: () => void;
}

export function ProviderDetailPage(props: ProviderDetailPageProps) {
  const [detailResult] = createResource(() => props.slug, async (slug) => {
    if (slug === null) {
      return undefined;
    }

    return getProviderDetail(slug);
  });
  const detail = createMemo(() => resultData(detailResult()));
  const images = createMemo(() => detail()?.images ?? []);
  const heroImage = createMemo(() => firstPublicImage(images()));
  const errorMessage = createMemo(() => {
    const result = detailResult();
    if (result === undefined || result.ok) {
      return null;
    }

    return result.error.message;
  });

  return (
    <section class="public-marketplace">
      <div class="profile-backbar">
        <button class="link-button" type="button" onClick={props.onBack}>
          Back to providers
        </button>
      </div>

      <Show when={errorMessage() === null} fallback={<ProfileState title="Profile unavailable" body={errorMessage() ?? "The provider profile could not be loaded."} />}>
        <Show when={!detailResult.loading} fallback={<ProfileState title="Loading profile" body="Fetching provider details and recent updates." />}>
          <Show when={detail()} fallback={<ProfileState title="Profile unavailable" body="This provider profile is not available." />}>
        {(loaded) => (
          <>
            <header class="profile-hero">
              <div class="profile-hero__media">
                <Show when={heroImage()?.public_url} fallback={<span>{profileInitials(loaded().profile.display_name)}</span>}>
                  {(url) => <img src={url()} alt="" />}
                </Show>
              </div>
              <div class="profile-hero__copy">
                <p class="eyebrow">{loaded().profile.service_area ?? "Service provider"}</p>
                <h1>{loaded().profile.display_name}</h1>
                <p class="profile-hero__headline">
                  {loaded().profile.headline ?? "Published marketplace profile"}
                </p>
                <div class="profile-hero__actions">
                  <button class="primary-button" type="button" onClick={props.onOpenPayments}>
                    Start payment
                  </button>
                  <button class="secondary-button" type="button" onClick={props.onBack}>
                    Compare providers
                  </button>
                </div>
                <div class="profile-facts" aria-label="Provider facts">
                  <Fact label="Status" value={loaded().profile.status} />
                  <Fact label="Moderation" value={loaded().profile.moderation_status} />
                  <Fact label="Updated" value={formatDate(loaded().profile.updated_at)} />
                </div>
              </div>
            </header>

            <div class="profile-content-grid">
              <section class="profile-section profile-section--main">
                <div>
                  <p class="eyebrow">Overview</p>
                  <h2>Service profile</h2>
                </div>
                <p>{loaded().profile.bio ?? "This provider has not added a long-form profile yet."}</p>
              </section>

              <aside class="profile-section">
                <div>
                  <p class="eyebrow">At a glance</p>
                  <h2>Provider details</h2>
                </div>
                <dl class="profile-detail-list">
                  <DetailTerm label="Service area" value={loaded().profile.service_area ?? "Not listed"} />
                  <DetailTerm label="Images" value={imageCountLabel(images())} />
                  <DetailTerm label="Blog posts" value={postCountLabel(loaded().blog_posts)} />
                </dl>
              </aside>
            </div>

            <section class="profile-section">
              <div class="profile-section__header">
                <div>
                  <p class="eyebrow">Updates</p>
                  <h2>Provider blog</h2>
                </div>
                <span class="marketplace-chip">{postCountLabel(loaded().blog_posts)}</span>
              </div>
              <Show
                when={loaded().blog_posts.length > 0}
                fallback={<p class="marketplace-empty">No published posts yet.</p>}
              >
                <div class="profile-blog-list">
                  <For each={loaded().blog_posts}>
                    {(post) => <BlogPreview post={post} />}
                  </For>
                </div>
              </Show>
            </section>
          </>
        )}
          </Show>
        </Show>
      </Show>
    </section>
  );
}

function firstPublicImage(images: readonly ImageResponse[]): ImageResponse | null {
  const image = images.find((candidate) => candidate.public_url !== null);
  if (image === undefined) {
    return null;
  }

  return image;
}

function profileInitials(displayName: string): string {
  const parts = displayName.split(" ").filter((part) => part.length > 0);
  const initials = parts
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase() ?? "")
    .join("");

  if (initials.length === 0) {
    return "SP";
  }

  return initials;
}

function formatDate(value: string): string {
  const parsed = new Date(value);
  if (Number.isNaN(parsed.getTime())) {
    return "Unknown";
  }

  return new Intl.DateTimeFormat("en", {
    day: "numeric",
    month: "short",
    year: "numeric"
  }).format(parsed);
}

function imageCountLabel(images: readonly ImageResponse[]): string {
  if (images.length === 1) {
    return "1 image";
  }

  return `${images.length} images`;
}

function postCountLabel(posts: readonly ProviderBlogPostResponse[]): string {
  if (posts.length === 1) {
    return "1 post";
  }

  return `${posts.length} posts`;
}

function Fact(props: { readonly label: string; readonly value: string }) {
  return (
    <div>
      <span>{props.label}</span>
      <strong>{props.value}</strong>
    </div>
  );
}

function DetailTerm(props: { readonly label: string; readonly value: string }) {
  return (
    <div>
      <dt>{props.label}</dt>
      <dd>{props.value}</dd>
    </div>
  );
}

function BlogPreview(props: { readonly post: ProviderBlogPostResponse }) {
  return (
    <article class="profile-blog-card">
      <div>
        <h3>{props.post.title}</h3>
        <p>{props.post.excerpt ?? "Published provider update"}</p>
      </div>
      <span>{props.post.published_at === null ? "Draft timing" : formatDate(props.post.published_at)}</span>
    </article>
  );
}

function ProfileState(props: { readonly title: string; readonly body: string }) {
  return (
    <div class="marketplace-state" role="status">
      <strong>{props.title}</strong>
      <p>{props.body}</p>
    </div>
  );
}
