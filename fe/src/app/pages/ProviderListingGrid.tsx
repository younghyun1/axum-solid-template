import { For, Show } from "solid-js";

import type { ProviderDirectoryCardResponse } from "../../api/marketplaceTypes";
import {
  compactHeadline,
  providerInitials,
  providerLocationLabel
} from "./providerDirectoryModel";

interface ProviderListingGridProps {
  readonly errorMessage: string | null;
  readonly loading: boolean;
  readonly providers: readonly ProviderDirectoryCardResponse[];
  readonly onOpenProvider: (slug: string) => void;
}

export function ProviderListingGrid(props: ProviderListingGridProps) {
  return (
    <Show
      when={props.errorMessage === null}
      fallback={
        <StatePanel
          title="Directory unavailable"
          body={props.errorMessage ?? "The directory could not be loaded."}
        />
      }
    >
      <Show
        when={!props.loading}
        fallback={<StatePanel title="Loading providers" body="Fetching published profiles." />}
      >
        <Show
          when={props.providers.length > 0}
          fallback={
            <StatePanel
              title="No matching providers"
              body="Adjust the search, location, or media filters to broaden the view."
            />
          }
        >
          <div class="template-card-fragment" id="directory-results">
            <For each={props.providers}>
              {(provider) => (
                <ProviderListingCard provider={provider} onOpenProvider={props.onOpenProvider} />
              )}
            </For>
          </div>
        </Show>
      </Show>
    </Show>
  );
}

function ProviderListingCard(props: {
  readonly provider: ProviderDirectoryCardResponse;
  readonly onOpenProvider: (slug: string) => void;
}) {
  const imageUrl = () => props.provider.primary_image?.public_url ?? null;
  return (
    <article class="template-listing template-profile-listing">
      <Show
        when={imageUrl()}
        fallback={
          <div class="template-listing-photo template-listing-photo--fallback">
            <span>{providerInitials(props.provider.display_name)}</span>
          </div>
        }
      >
        {(url) => <img class="template-listing-photo" src={url()} alt="" loading="lazy" />}
      </Show>

      <div class="template-listing-top">
        <div class="template-icon" aria-hidden="true">
          <DiamondIcon />
        </div>
        <span class="template-tag">{imageUrl() === null ? "Media pending" : "Verified media"}</span>
      </div>

      <div class="template-listing-copy">
        <h3>{props.provider.display_name}</h3>
        <p>{compactHeadline(props.provider)}</p>
      </div>

      <div class="template-listing-meta">
        <span>{providerLocationLabel(props.provider.subdivision)}</span>
        <span>Published profile</span>
      </div>

      <button
        class="template-contact-button"
        type="button"
        onClick={() => props.onOpenProvider(props.provider.slug)}
      >
        <MessageIcon />
        <span>View profile</span>
      </button>
    </article>
  );
}

function StatePanel(props: { readonly title: string; readonly body: string }) {
  return (
    <div class="marketplace-state" role="status">
      <strong>{props.title}</strong>
      <p>{props.body}</p>
    </div>
  );
}

function DiamondIcon() {
  return (
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
      <path d="M12 3 4 9l8 12 8-12-8-6Z" stroke="currentColor" stroke-width="1.7" />
      <path d="m4 9 8 3 8-3" stroke="currentColor" stroke-width="1.7" />
    </svg>
  );
}

function MessageIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
      <path
        d="M21 11.5a8.4 8.4 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.4 8.4 0 0 1-3.8-.9L3 21l1.9-5.7a8.4 8.4 0 0 1-.9-3.8 8.5 8.5 0 1 1 17 0Z"
        stroke="currentColor"
        stroke-linejoin="round"
        stroke-width="1.8"
      />
    </svg>
  );
}
