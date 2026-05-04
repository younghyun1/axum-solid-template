import { createMemo, createResource, For, Show } from "solid-js";

import { getProviderDetail } from "../../api/marketplaceApi";
import { resultData } from "../helpers";

interface ProviderDetailPageProps {
  readonly slug: string | null;
  readonly onBack: () => void;
}

export function ProviderDetailPage(props: ProviderDetailPageProps) {
  const [detailResult] = createResource(() => props.slug, async (slug) => {
    if (slug === null) {
      return undefined;
    }

    return getProviderDetail(slug);
  });
  const detail = createMemo(() => resultData(detailResult()));

  return (
    <section class="marketplace-layout">
      <button class="link-button" type="button" onClick={props.onBack}>
        Back to providers
      </button>

      <Show when={detail()} fallback={<p class="marketplace-empty">Provider profile unavailable.</p>}>
        {(loaded) => (
          <>
            <div class="provider-detail">
              <div class="provider-detail__media">
                <Show when={loaded().images[0]?.public_url} fallback={<span>Provider</span>}>
                  {(url) => <img src={url()} alt="" loading="lazy" />}
                </Show>
              </div>
              <div class="provider-detail__copy">
                <p class="eyebrow">{loaded().profile.service_area ?? "Service provider"}</p>
                <h1>{loaded().profile.display_name}</h1>
                <p>{loaded().profile.headline ?? "Published marketplace profile"}</p>
                <p>{loaded().profile.bio ?? ""}</p>
              </div>
            </div>

            <section class="marketplace-panel">
              <h2>Provider blog</h2>
              <Show
                when={loaded().blog_posts.length > 0}
                fallback={<p class="marketplace-empty">No published posts yet.</p>}
              >
                <div class="marketplace-list">
                  <For each={loaded().blog_posts}>
                    {(post) => (
                      <article class="marketplace-row">
                        <div>
                          <strong>{post.title}</strong>
                          <p>{post.excerpt ?? "Published provider update"}</p>
                        </div>
                        <span class="marketplace-chip">{post.status}</span>
                      </article>
                    )}
                  </For>
                </div>
              </Show>
            </section>
          </>
        )}
      </Show>
    </section>
  );
}
