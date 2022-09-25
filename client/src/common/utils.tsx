import camelcaseKeys from "camelcase-keys";
import snakecaseKeys from "snakecase-keys";
import { Buffer } from "buffer";

export function camelCaseObject<T extends Record<string, any> | readonly any[]>(
  obj: T
) {
  return camelcaseKeys(obj, { deep: true });
}
export function snakeCaseObject<T extends Record<string, any> | readonly any[]>(
  obj: T
) {
  return snakecaseKeys(obj, { deep: true });
}

export function basicAuthHeader(username: string, password: string): string {
  const base64UsernamePassword: string = Buffer.from(
    `${username}:${password}`
  ).toString("base64");

  const header = `Basic ${base64UsernamePassword}`;

  return header;
}

export function bearerAuthHeader(token: string): string {
  return `Bearer ${token}`;
}
