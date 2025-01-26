import * as Z from "zod";

export const post = async <T, R>(
  url: string,
  data: T,
  validator: Z.ZodType<R>
): Promise<R> => {
  const response = await fetch(url, {
    headers: { "content-type": "application/json" },
    method: "post",
    body: JSON.stringify(data),
  });
  const json = await response.json();
  return validator.parse(json);
};

// TODO: Investigate storing the token in storage
export const authenticatedPost = async <T, R>(
  url: string,
  token: string,
  data: T,
  validator: Z.ZodType<R>
): Promise<R> => {
  const response = await fetch(url, {
    headers: {
      "content-type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    method: "post",
    body: JSON.stringify(data),
  });
  const json = await response.json();
  return validator.parse(json);
};

export const get = async <R>(
  url: string,
  validator: Z.ZodType<R>
): Promise<R> => {
  const response = await fetch(url, {
    headers: { "content-type": "application/json" },
    method: "get",
  });
  const json = await response.json();
  return validator.parse(json);
};

// TODO: Investigate storing the token in storage
export const authenticatedGet = async <R>(
  url: string,
  token: string,
  validator: Z.ZodType<R>
): Promise<R> => {
  const response = await fetch(url, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
    method: "get",
  });
  const json = await response.json();
  return validator.parse(json);
};

export const authenticatedGetFactory =
  <R>(url: string, validator: Z.ZodType<R>) =>
  (token: string) =>
    authenticatedGet(url, token, validator);
