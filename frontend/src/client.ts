import * as API from "./tanstack-client";
import { requestFn, RequestFnResponse } from "@openapi-qraft/react";
import { QraftSecureRequestFn } from "@openapi-qraft/react/Unstable_QraftSecureRequestFn";
import { QueryClient } from "@tanstack/react-query";

const queryClient = new QueryClient();

let authorisationToken: string | null = null;
export const setAuthorisationToken = (token: string) => {
  authorisationToken = token;
};

export const client = API.createAPIClient({
  requestFn: Q,
  baseUrl: location.origin,
  queryClient,
});

export const throwError = <T, E>(response: RequestFnResponse<T, E>) => {
  if (typeof response.error !== "undefined") {
    throw response.error;
  }
  return response.data as T;
};
