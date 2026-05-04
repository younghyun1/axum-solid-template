import type { Crepe } from "@milkdown/crepe";
import { createEffect, onCleanup, onMount } from "solid-js";

interface MarkdownEditorProps {
  readonly value: string;
  readonly label: string;
  readonly resetToken: number;
  readonly readonly?: boolean;
  readonly onChange: (value: string) => void;
}

export function MarkdownEditor(props: MarkdownEditorProps) {
  let rootElement: HTMLDivElement | undefined;
  let editor: Crepe | null = null;
  let activeResetToken = props.resetToken;

  const destroyEditor = () => {
    if (editor === null) {
      return;
    }

    editor.destroy();
    editor = null;
  };

  const createEditor = async () => {
    if (rootElement === undefined) {
      return;
    }

    destroyEditor();
    const [{ Crepe }] = await Promise.all([
      import("@milkdown/crepe"),
      import("@milkdown/crepe/theme/common/style.css"),
      import("@milkdown/crepe/theme/frame.css")
    ]);
    rootElement.replaceChildren();
    const nextEditor = new Crepe({
      root: rootElement,
      defaultValue: props.value,
      features: {
        "image-block": false,
        "top-bar": true
      }
    });
    nextEditor.on((listener) => {
      listener.markdownUpdated((_ctx, markdown) => {
        props.onChange(markdown);
      });
    });
    editor = nextEditor;
    await nextEditor.create();
    nextEditor.setReadonly(props.readonly === true);
  };

  onMount(() => {
    void createEditor();
  });

  createEffect(() => {
    const readonly = props.readonly === true;
    if (editor !== null) {
      editor.setReadonly(readonly);
    }
  });

  createEffect(() => {
    if (props.resetToken === activeResetToken) {
      return;
    }

    activeResetToken = props.resetToken;
    void createEditor();
  });

  onCleanup(() => {
    destroyEditor();
  });

  return (
    <div class="markdown-editor">
      <span class="markdown-editor__label">{props.label}</span>
      <div ref={rootElement} class="markdown-editor__surface" />
    </div>
  );
}
