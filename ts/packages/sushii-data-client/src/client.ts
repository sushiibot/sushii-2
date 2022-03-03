import {
  TransportGuildConfig,
  TransportGuildConfigModel,
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
    const response = await this.fetch(`/guild-configs/${guildId}`);
    return TransportGuildConfig.parse(await response.json());
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
    await this.fetch(`/guild-configs/${guildId}`, {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(config),
    });
  }
}
