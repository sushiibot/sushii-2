import {
  TransportGuildConfig,
  TransportGuildConfigModel,
  TransportUser,
  TransportUserModel,
} from "@sushiibot/sushii-data/src/client";
import { Agent, AgentOptions } from "http";
import fetch, { RequestInit, Response } from "node-fetch";

const defaultAgentOptions: AgentOptions = {
  keepAlive: true,
};

export default class ApiClient {
  endpoint: string;
  agent: Agent;

  constructor(endpoint: string, agentOpts: AgentOptions = defaultAgentOptions) {
    // Remove trailing slash
    this.endpoint = endpoint.replace(/\/$/, "");
    this.agent = new Agent(agentOpts);
  }

  /**
   * Custom fetch method that overrides node-fetch with the custom client agent
   *
   * @param url
   * @param init
   * @returns {Promise<Response>}
   */
  private async fetch(path: string, init?: RequestInit): Promise<Response> {
    const url = this.endpoint + path;

    return fetch(url, {
      agent: this.agent,
      ...init,
    });
  }

  /**
   * Gets a guild's config
   *
   * @param guildId
   * @returns {Promise<TransportGuildConfigModel>}
   */
  public async getGuildConfig(
    guildId: string
  ): Promise<TransportGuildConfigModel> {
    const res = await this.fetch(`/guild-configs/${guildId}`);
    await checkErr(res);

    return TransportGuildConfig.parse(await res.json());
  }

  /**
   * Updates a guild config, must contain all fields -- not a partial update.
   *
   * @param guildId Unique guild ID
   * @param config
   */
  public async updateGuildConfig(
    guildId: string,
    config: TransportGuildConfigModel
  ): Promise<void> {
    const res = await this.fetch(`/guild-configs/${guildId}`, {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(config),
    });

    await checkErr(res);
  }

  /**
   * Gets a user
   *
   * @param userId
   * @returns {Promise<TransportUserModel>}
   */
  public async getUser(userId: string): Promise<TransportUserModel> {
    const res = await this.fetch(`/users/${userId}`);
    await checkErr(res);
    return TransportUser.parse(await res.json());
  }

  /**
   * Updates a user
   *
   * @param userId
   * @returns {Promise<TransportUserModel>}
   */
  public async updateUser(user: TransportUserModel): Promise<void> {
    const res = await this.fetch(`/users/${user.id}`, {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(user),
    });

    await checkErr(res);
  }
}

async function checkErr(res: Response) {
  if (!res.ok) {
    const jsonErr = await res.json();

    throw new Error(`${res.status} ${res.statusText}: ${jsonErr.message}`);
  }
}
