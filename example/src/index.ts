import { useGrpcRequest } from "@aboutyou/data-fetcher";
import { CategoryStreamService_GetProductStream } from "@aboutyou/grpc-clients/tadarida/aysa_api/services/category_stream/v1/stream/service";
import { CustomerService_getCustomer } from "@aboutyou/grpc-clients/grips/aysa_api/services/category_stream/v1/stream/service";

const useProductsByPage = (page: number) => {
  return useGrpcRequest(
    CategoryStreamService_GetProductStream,
    {},
    {
      staleTime: 30000,
    }
  );
};

const useProductsByPage2 = (page: number) => {
  return useGrpcRequest(CategoryStreamService_GetProductStream, {});
};

const config = {};
const useProductsByPage3 = (page: number) => {
  return useGrpcRequest(CategoryStreamService_GetProductStream, {}, config);
};

const useLegacyRequestExample = (page: number) => {
  return useLegacyGrpcRequest(
    CustomerService_getCustomer,
    { page },
    { staleTime: Infinity }
  );
};

/*
/ example useUpdateGrpcRequestCache used inside useGenericGrpcRequest
*/
function useGenericGrpcRequest(
  maybeAsyncService,
  params,
  genericConfig,
  grpcQueryConfig
) {
  const fetch = useGenericGrpcRequestFn(
    maybeAsyncService,
    params,
    genericConfig
  );

  const updateCache = useUpdateGrpcRequestCache(
    maybeAsyncService,
    params,
    grpcQueryConfig
  );

  const request = useRequest(key, fetch, queryConfig);
  request.updateCache = updateCache;

  return request;
}
