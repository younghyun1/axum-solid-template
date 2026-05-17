import { For, Show } from "solid-js";

import type { DirectorySection } from "./providerDirectoryModel";
import { sectionVisible } from "./providerDirectoryModel";

interface DirectoryFeature {
  readonly cta: string;
  readonly description: string;
  readonly label: string;
  readonly title: string;
}

interface ProviderDirectorySectionsProps {
  readonly activeSection: DirectorySection;
  readonly onSectionChange: (section: DirectorySection) => void;
}

const GUIDE_ITEMS: readonly DirectoryFeature[] = [
  {
    cta: "Read guide",
    description: "How to compare scope, timing, and expectations before opening a request.",
    label: "Guide",
    title: "Choosing the right provider"
  },
  {
    cta: "Start quiz",
    description: "A short checklist for budget, timeline, preferred format, and communication style.",
    label: "Quiz",
    title: "Request readiness check"
  },
  {
    cta: "Read update",
    description: "New profile media, search improvements, and public listing quality notes.",
    label: "Update",
    title: "Marketplace release notes"
  }
];

const EVENT_ITEMS: readonly DirectoryFeature[] = [
  {
    cta: "Register interest",
    description: "A moderated introduction session for new members and published providers.",
    label: "Workshop",
    title: "Provider discovery salon"
  },
  {
    cta: "View details",
    description: "A structured networking listing for verified accounts and service teams.",
    label: "Meetup",
    title: "Members mixer"
  },
  {
    cta: "Watch live",
    description: "Panel discussion on remote services, booking clarity, and profile trust signals.",
    label: "Live",
    title: "Live panel: service design"
  }
];

export function ProviderDirectorySections(props: ProviderDirectorySectionsProps) {
  return (
    <>
      <Show when={sectionVisible(props.activeSection, "live")}>
        <article class="template-listing template-stream-card" id="live-sessions">
          <div class="template-listing-top">
            <div class="template-icon template-live-icon" aria-hidden="true">
              <LiveIcon />
            </div>
            <span class="template-tag">
              <span class="template-live-signal" aria-hidden="true" />
              Live
            </span>
          </div>
          <h3>Live session board</h3>
          <p>
            Scheduled remote consultations, open office hours, private requests, and upcoming
            provider-led sessions.
          </p>
          <div class="template-listing-meta">
            <span>Remote</span>
            <span>Scheduled sessions</span>
            <span>Provider-led</span>
          </div>
          <button
            class="template-contact-button template-live-contact"
            type="button"
            onClick={() => props.onSectionChange("profiles")}
          >
            <LiveIcon />
            <span>Browse providers</span>
          </button>
        </article>
      </Show>

      <Show when={sectionVisible(props.activeSection, "guides")}>
        <FeatureSection
          id="guides"
          description="Guides, checklists, quizzes, and short reads for clearer provider selection."
          features={GUIDE_ITEMS}
          label="Guides"
          meta={["Guides", "Quizzes", "Account rewards"]}
          onAction={() => props.onSectionChange("profiles")}
          title="Marketplace learning"
        />
      </Show>

      <Show when={sectionVisible(props.activeSection, "events")}>
        <FeatureSection
          id="events"
          description="Public and member-only listings for workshops, talks, live panels, and launch nights."
          features={EVENT_ITEMS}
          label="Events"
          meta={["Events", "Members", "Remote options"]}
          onAction={() => props.onSectionChange("profiles")}
          title="Upcoming events"
        />
      </Show>
    </>
  );
}

function FeatureSection(props: {
  readonly description: string;
  readonly features: readonly DirectoryFeature[];
  readonly id: string;
  readonly label: string;
  readonly meta: readonly string[];
  readonly onAction: () => void;
  readonly title: string;
}) {
  return (
    <article class="template-listing template-feature-section" id={props.id}>
      <div class="template-listing-top">
        <div class="template-icon" aria-hidden="true">
          <DocumentIcon />
        </div>
        <span class="template-tag">{props.label}</span>
      </div>
      <h3>{props.title}</h3>
      <p>{props.description}</p>
      <div class="template-feature-list">
        <For each={props.features}>
          {(feature) => (
            <article class="template-feature-card">
              <span class="template-tag">{feature.label}</span>
              <h4>{feature.title}</h4>
              <p>{feature.description}</p>
              <button class="template-article-link" type="button" onClick={props.onAction}>
                {feature.cta}
              </button>
            </article>
          )}
        </For>
      </div>
      <div class="template-listing-meta">
        <For each={props.meta}>{(value) => <span>{value}</span>}</For>
      </div>
    </article>
  );
}

function LiveIcon() {
  return (
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
      <path d="M4 7h16v10H4V7Z" stroke="currentColor" stroke-width="1.7" />
      <path d="m10 10 5 2-5 2v-4Z" fill="currentColor" />
    </svg>
  );
}

function DocumentIcon() {
  return (
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
      <path d="M5 4h14v16H5V4Z" stroke="currentColor" stroke-width="1.7" />
      <path d="M8 8h8M8 12h8M8 16h5" stroke="currentColor" stroke-linecap="round" stroke-width="1.7" />
    </svg>
  );
}
