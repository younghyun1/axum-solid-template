import { createMemo, createResource, createSignal, For, Show } from "solid-js";

import {
  getPaymentIntents,
  getUserMarketplaceProfile,
  upsertUserMarketplaceProfile
} from "../../api/marketplaceApi";
import type { MeResponse } from "../../api/types";
import { resultData } from "../helpers";

interface UserMarketplacePageProps {
  readonly profile: MeResponse | null;
  readonly onSignIn: () => void;
}

export function UserMarketplacePage(props: UserMarketplacePageProps) {
  const [saveTick, setSaveTick] = createSignal(0);
  const [profileResult] = createResource(saveTick, getUserMarketplaceProfile);
  const [paymentsResult] = createResource(getPaymentIntents);
  const marketplaceProfile = createMemo(() => resultData(profileResult()));
  const paymentIntents = createMemo(() => resultData(paymentsResult())?.payment_intents ?? []);

  const [displayName, setDisplayName] = createSignal("");
  const [bio, setBio] = createSignal("");
  const [notice, setNotice] = createSignal("");

  const saveProfile = async () => {
    const result = await upsertUserMarketplaceProfile({
      display_name: displayName().trim() || null,
      bio: bio().trim() || null,
      phone: null,
      public_email: null
    });
    if (result.ok) {
      setNotice("Profile saved.");
      setSaveTick((value) => value + 1);
      return;
    }

    setNotice(result.error.message);
  };

  return (
    <section class="marketplace-layout">
      <Show
        when={props.profile !== null}
        fallback={
          <div class="marketplace-panel">
            <h1>User workspace</h1>
            <p class="marketplace-empty">Sign in to manage your profile and payments.</p>
            <button class="primary-button" type="button" onClick={props.onSignIn}>
              Sign in
            </button>
          </div>
        }
      >
        <div class="marketplace-heading">
          <p class="eyebrow">User workspace</p>
          <h1>{props.profile?.user_info.user_name}</h1>
        </div>

        <div class="marketplace-columns">
          <section class="marketplace-panel">
            <h2>Profile</h2>
            <p class="marketplace-empty">
              Current display name: {marketplaceProfile()?.display_name ?? "Not set"}
            </p>
            <div class="flow-form">
              <input
                placeholder="Display name"
                value={displayName()}
                onInput={(event) => setDisplayName(event.currentTarget.value)}
              />
              <textarea
                placeholder="Public bio"
                value={bio()}
                onInput={(event) => setBio(event.currentTarget.value)}
              />
              <button class="primary-button" type="button" onClick={saveProfile}>
                Save profile
              </button>
              <Show when={notice()}>
                {(text) => <p class="field-note">{text()}</p>}
              </Show>
            </div>
          </section>

          <section class="marketplace-panel">
            <h2>Payment intents</h2>
            <Show
              when={paymentIntents().length > 0}
              fallback={<p class="marketplace-empty">No payment intents created yet.</p>}
            >
              <div class="marketplace-list">
                <For each={paymentIntents()}>
                  {(intent) => (
                    <div class="marketplace-row">
                      <div>
                        <strong>{intent.amount_minor_units} minor units</strong>
                        <p>{intent.payment_provider}</p>
                      </div>
                      <span class="marketplace-chip">{intent.status}</span>
                    </div>
                  )}
                </For>
              </div>
            </Show>
          </section>
        </div>
      </Show>
    </section>
  );
}
