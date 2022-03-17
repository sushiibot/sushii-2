import { ApiClient } from "@sushiibot/sushii-data-client";
import CDNClient from "./cdn";
import { ConfigI } from "./config";
import RESTClient from "./rest";

export default class Context {
  public readonly sushiiAPI: ApiClient;

  public readonly REST: RESTClient;

  public readonly CDN: CDNClient;

  constructor(config: ConfigI) {
    this.sushiiAPI = new ApiClient(config.dataApiURL);
    this.REST = new RESTClient(config);
    this.CDN = new CDNClient();
  }
}
