import {
  RESTPostAPIChatInputApplicationCommandsJSONBody,
  RESTPostAPIApplicationCommandsJSONBody,
  APIChatInputApplicationCommandInteraction,
} from "discord-api-types/v9";
import Context from "../../context";
import InteractionHandler from "./InteractionHandler";

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
    interaction: APIChatInputApplicationCommandInteraction
  ): Promise<void>;
}
