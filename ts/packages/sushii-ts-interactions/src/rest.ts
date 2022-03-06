import { REST } from "@discordjs/rest";
import {
  Routes,
  RESTPostAPIInteractionCallbackJSONBody,
  APIInteractionResponseCallbackData,
  InteractionResponseType,
} from "discord-api-types/v9";
import { ConfigI } from "./config";

export default class RESTClient {
  private rest: REST;

  constructor(private readonly config: ConfigI) {
    this.rest = new REST({
      api: this.config.proxyUrl,
    });
  }

  public async interactionReplyMsg(msg: APIInteractionResponseCallbackData) {
    await this.interactionCallback({
      type: InteractionResponseType.ChannelMessageWithSource,
      data: msg,
    });
  }

  public async interactionCallback(
    payload: RESTPostAPIInteractionCallbackJSONBody
  ) {
    await this.rest.post(
      Routes.applicationCommands(this.config.applicationId),
      { body: payload }
    );
  }
}
