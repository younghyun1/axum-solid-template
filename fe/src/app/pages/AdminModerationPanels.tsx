import { createSignal } from "solid-js";

import { moderateProviderBlogPost, moderateProviderProfile } from "../../api/marketplaceApi";
import type { ModerationStatus } from "../../api/marketplaceTypes";

interface AdminModerationPanelsProps {
  readonly onNotice: (message: string) => void;
  readonly onRefresh: () => void;
}

export function AdminModerationPanels(props: AdminModerationPanelsProps) {
  const [providerModerationId, setProviderModerationId] = createSignal("");
  const [providerModerationStatus, setProviderModerationStatus] =
    createSignal<ModerationStatus>("approved");
  const [blogModerationId, setBlogModerationId] = createSignal("");
  const [blogModerationStatus, setBlogModerationStatus] =
    createSignal<ModerationStatus>("approved");

  const submitProviderModeration = async () => {
    const result = await moderateProviderProfile(providerModerationId().trim(), {
      moderation_status: providerModerationStatus()
    });
    props.onNotice(result.ok ? "Provider moderation updated." : result.error.message);
    if (result.ok) {
      props.onRefresh();
      setProviderModerationId("");
    }
  };

  const submitBlogModeration = async () => {
    const result = await moderateProviderBlogPost(blogModerationId().trim(), {
      moderation_status: blogModerationStatus()
    });
    props.onNotice(result.ok ? "Blog moderation updated." : result.error.message);
    if (result.ok) {
      props.onRefresh();
      setBlogModerationId("");
    }
  };

  return (
    <div class="marketplace-columns">
      <section class="marketplace-panel">
        <h2>Moderate provider</h2>
        <div class="flow-form">
          <input
            placeholder="Provider profile UUID"
            value={providerModerationId()}
            onInput={(event) => setProviderModerationId(event.currentTarget.value)}
          />
          <select
            value={providerModerationStatus()}
            onInput={(event) =>
              setProviderModerationStatus(event.currentTarget.value as ModerationStatus)
            }
          >
            <option value="approved">Approve</option>
            <option value="pending">Mark pending</option>
            <option value="rejected">Reject</option>
          </select>
          <button class="secondary-button" type="button" onClick={submitProviderModeration}>
            Apply provider decision
          </button>
        </div>
      </section>

      <section class="marketplace-panel">
        <h2>Moderate provider post</h2>
        <div class="flow-form">
          <input
            placeholder="Provider blog post UUID"
            value={blogModerationId()}
            onInput={(event) => setBlogModerationId(event.currentTarget.value)}
          />
          <select
            value={blogModerationStatus()}
            onInput={(event) =>
              setBlogModerationStatus(event.currentTarget.value as ModerationStatus)
            }
          >
            <option value="approved">Approve</option>
            <option value="pending">Mark pending</option>
            <option value="rejected">Reject</option>
          </select>
          <button class="secondary-button" type="button" onClick={submitBlogModeration}>
            Apply post decision
          </button>
        </div>
      </section>
    </div>
  );
}
