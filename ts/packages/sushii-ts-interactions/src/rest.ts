import { REST } from "@discordjs/rest";
import {
  Routes,
  RESTPostAPIInteractionCallbackJSONBody,
  APIInteractionResponseCallbackData,
  InteractionResponseType,
  RESTGetAPIUserResult,
  RESTGetAPIGuildMemberResult,
} from "discord-api-types/v9";
import { ConfigI } from "./config";

export default class RESTClient {
  private rest: REST;

  constructor(private readonly config: ConfigI) {
    this.rest = new REST({
      api: this.config.proxyUrl,
    }).setToken(config.token);
  }

  public async interactionReplyMsg(
    interactionId: string,
    interactionToken: string,
    msg: APIInteractionResponseCallbackData
  ): Promise<void> {
    await this.interactionCallback(interactionId, interactionToken, {
      type: InteractionResponseType.ChannelMessageWithSource,
      data: msg,
    });
  }

  public async interactionCallback(
    interactionId: string,
    interactionToken: string,
    payload: RESTPostAPIInteractionCallbackJSONBody
  ): Promise<void> {
    await this.rest.post(
      Routes.interactionCallback(interactionId, interactionToken),
      { body: payload }
    );
  }

  public getUser(userId: string): Promise<RESTGetAPIUserResult> {
    return this.rest.get(Routes.user(userId)) as Promise<RESTGetAPIUserResult>;
  }

  public getMember(
    guildId: string,
    userId: string
  ): Promise<RESTGetAPIGuildMemberResult> {
    return this.rest.get(
      Routes.guildMember(guildId, userId)
    ) as Promise<RESTGetAPIGuildMemberResult>;
  }
}
