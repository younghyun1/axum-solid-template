export type FieldKind = "email" | "number" | "password" | "text";

export interface DemoField {
  readonly key: string;
  readonly label: string;
  readonly kind: FieldKind;
  readonly initialValue: string;
  readonly optional?: boolean;
}

export type FormValues = Record<string, string>;

export type ValueResult<TValue> =
  | {
      readonly ok: true;
      readonly value: TValue;
    }
  | {
      readonly ok: false;
      readonly message: string;
    };

export function field(
  key: string,
  label: string,
  kind: FieldKind,
  initialValue = "",
  optional?: boolean
): DemoField {
  const demoField: DemoField = {
    key,
    label,
    kind,
    initialValue
  };

  if (optional !== undefined) {
    return {
      ...demoField,
      optional
    };
  }

  return demoField;
}

export function requiredValue(values: FormValues, key: string, label: string): ValueResult<string> {
  const value = values[key]?.trim() ?? "";
  if (value.length === 0) {
    return {
      ok: false,
      message: `${label} is required`
    };
  }

  return {
    ok: true,
    value
  };
}

export function requiredInteger(values: FormValues, key: string, label: string): ValueResult<number> {
  const rawValue = requiredValue(values, key, label);
  if (!rawValue.ok) {
    return rawValue;
  }

  return parseInteger(rawValue.value, label);
}

export function optionalInteger(
  values: FormValues,
  key: string,
  label: string
): ValueResult<number | null> {
  const rawValue = values[key]?.trim() ?? "";
  if (rawValue.length === 0) {
    return {
      ok: true,
      value: null
    };
  }

  return parseInteger(rawValue, label);
}

function parseInteger(rawValue: string, label: string): ValueResult<number> {
  const value = Number.parseInt(rawValue, 10);
  if (!Number.isSafeInteger(value) || value.toString() !== rawValue) {
    return {
      ok: false,
      message: `${label} must be an integer`
    };
  }

  return {
    ok: true,
    value
  };
}
