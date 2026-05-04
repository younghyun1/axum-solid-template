import { createMemo, createResource, createSignal, For, Show } from "solid-js";

import {
  createBan,
  createBanner,
  getActiveBans,
  getAdminMarketplaceOverview
} from "../../api/marketplaceApi";
import type { MeResponse } from "../../api/types";
import { resultData } from "../helpers";

interface AdminMarketplacePageProps {
  readonly profile: MeResponse | null;
}

export function AdminMarketplacePage(props: AdminMarketplacePageProps) {
  const canModerate = createMemo(() => {
    const role = props.profile?.claims.role_type;
    return role === "admin" || role === "moderator";
  });
  const [refreshTick, setRefreshTick] = createSignal(0);
  const [overviewResult] = createResource(refreshTick, getAdminMarketplaceOverview);
  const [bansResult] = createResource(refreshTick, getActiveBans);
  const overview = createMemo(() => resultData(overviewResult()));
  const bans = createMemo(() => resultData(bansResult())?.bans ?? []);
  const [targetUserId, setTargetUserId] = createSignal("");
  const [banReason, setBanReason] = createSignal("");
  const [bannerTitle, setBannerTitle] = createSignal("");
  const [bannerUrl, setBannerUrl] = createSignal("");
  const [notice, setNotice] = createSignal("");

  const submitBan = async () => {
    const result = await createBan({
      target_user_id: targetUserId().trim(),
      scope: "account",
      reason: banReason().trim(),
      starts_at: null,
      expires_at: null
    });
    setNotice(result.ok ? "Ban created." : result.error.message);
    if (result.ok) {
      setRefreshTick((value) => value + 1);
      setTargetUserId("");
      setBanReason("");
    }
  };

  const submitBanner = async () => {
    const result = await createBanner({
      placement: "homepage_top",
      status: "active",
      title: bannerTitle().trim(),
      target_url: bannerUrl().trim(),
      priority: 0,
      starts_at: new Date().toISOString(),
      ends_at: null
    });
    setNotice(result.ok ? "Banner created." : result.error.message);
    if (result.ok) {
      setRefreshTick((value) => value + 1);
      setBannerTitle("");
      setBannerUrl("");
    }
  };

  return (
    <section class="marketplace-layout">
      <Show when={canModerate()} fallback={<p class="marketplace-empty">Moderator role required.</p>}>
        <div class="marketplace-heading">
          <p class="eyebrow">Marketplace admin</p>
          <h1>Moderation dashboard</h1>
        </div>

        <div class="metric-grid">
          <div class="metric-tile">
            <span>{overview()?.provider_count ?? 0}</span>
            <p>Providers</p>
          </div>
          <div class="metric-tile">
            <span>{overview()?.active_ban_count ?? 0}</span>
            <p>Active bans</p>
          </div>
          <div class="metric-tile">
            <span>{overview()?.payment_intent_count ?? 0}</span>
            <p>Payment intents</p>
          </div>
          <div class="metric-tile">
            <span>{overview()?.active_banner_count ?? 0}</span>
            <p>Active banners</p>
          </div>
        </div>

        <div class="marketplace-columns">
          <section class="marketplace-panel">
            <h2>Create ban</h2>
            <div class="flow-form">
              <input placeholder="Target user UUID" value={targetUserId()} onInput={(event) => setTargetUserId(event.currentTarget.value)} />
              <input placeholder="Reason" value={banReason()} onInput={(event) => setBanReason(event.currentTarget.value)} />
              <button class="danger-button" type="button" onClick={submitBan}>Ban account</button>
            </div>
          </section>

          <section class="marketplace-panel">
            <h2>Create banner</h2>
            <div class="flow-form">
              <input placeholder="Banner title" value={bannerTitle()} onInput={(event) => setBannerTitle(event.currentTarget.value)} />
              <input placeholder="Target URL" value={bannerUrl()} onInput={(event) => setBannerUrl(event.currentTarget.value)} />
              <button class="secondary-button" type="button" onClick={submitBanner}>Publish banner</button>
            </div>
          </section>
        </div>

        <Show when={notice()}>
          {(text) => <p class="field-note">{text()}</p>}
        </Show>

        <section class="marketplace-panel">
          <h2>Active bans</h2>
          <div class="marketplace-list">
            <For each={bans()}>
              {(ban) => (
                <div class="marketplace-row">
                  <div>
                    <strong>{ban.target_user_id}</strong>
                    <p>{ban.reason}</p>
                  </div>
                  <span class="marketplace-chip">{ban.scope}</span>
                </div>
              )}
            </For>
          </div>
        </section>
      </Show>
    </section>
  );
}
