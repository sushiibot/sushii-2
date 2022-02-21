import {
  RESTPostAPIChatInputApplicationCommandsJSONBody,
  RESTPostAPIApplicationCommandsJSONBody,
} from "discord-api-types/v9";
import { CommandInteraction } from "discord.js";

export interface SlashCommand {
  command:
    | RESTPostAPIChatInputApplicationCommandsJSONBody
    | RESTPostAPIApplicationCommandsJSONBody;
  handler: (ctx: any, interaction: CommandInteraction) => Promise<void>;
}
