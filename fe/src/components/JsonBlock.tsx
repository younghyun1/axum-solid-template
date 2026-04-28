interface JsonBlockProps {
  readonly value: unknown;
}

export function JsonBlock(props: JsonBlockProps) {
  return <pre class="json-block">{JSON.stringify(props.value, null, 2)}</pre>;
}
