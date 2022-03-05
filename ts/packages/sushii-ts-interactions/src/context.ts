import { ApiClient } from "@sushiibot/sushii-data-client";

export default class Context {
  api: ApiClient;

  constructor(endpoint: string) {
    this.api = new ApiClient(endpoint);
  }
}
