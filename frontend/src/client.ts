import {
  OperationSchema,
  RequestFnInfo,
  RequestFnOptions,
  RequestFnResponse,
  mergeHeaders,
  requestFn,
} from "@openapi-qraft/react";
import { QueryClient } from "@tanstack/react-query";
import { createAPIClient } from "./tanstack-client";

const queryClient = new QueryClient();

let authorisationToken: string | null = null;
export const setAuthorisationToken = (token: string) => {
  authorisationToken = token;
};

export const throwError = <T, E>(response: RequestFnResponse<T, E>) => {
  if (typeof response.error !== "undefined") {
    throw response.error;
  }
  return response.data as T;
};

function secureRequestFn<TData, TError>(
  schema: OperationSchema,
  requestInfo: RequestFnInfo,
  options?: RequestFnOptions
): Promise<RequestFnResponse<TData, TError>> {
  return requestFn(
    schema,
    typeof authorisationToken === "string"
      ? {
          ...requestInfo,
          headers: mergeHeaders(requestInfo.headers, {
            Authorization: `Bearer ${authorisationToken}`,
          }),
        }
      : requestInfo,
    options
  );
}

export const client = createAPIClient({
  requestFn: secureRequestFn,
  queryClient,
  baseUrl: location.origin,
});
