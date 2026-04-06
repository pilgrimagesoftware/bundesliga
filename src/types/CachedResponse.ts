export interface CachedResponse<T> {
  data: T;
  cached: boolean;
  next_refresh_at: number | null;
}
