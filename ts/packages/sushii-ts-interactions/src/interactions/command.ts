import {
  RESTPostAPIChatInputApplicationCommandsJSONBody,
  RESTPostAPIApplicationCommandsJSONBody,
} from "discord-api-types/v9";
import { CommandInteraction } from "discord.js";
import Context from "../context";
import InteractionHandler from "./interaction";

export default abstract class SlashCommandHandler extends InteractionHandler {
  /**
   * Data for command, e.g. the name, description, options
   */
  abstract readonly command:
    | RESTPostAPIChatInputApplicationCommandsJSONBody
    | RESTPostAPIApplicationCommandsJSONBody;

  /**
   * Field for the actual handler function
   */
  abstract handler(
    ctx: Context,
    interaction: CommandInteraction
  ): Promise<void>;
}
