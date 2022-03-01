import {
  RESTPostAPIChatInputApplicationCommandsJSONBody,
  RESTPostAPIApplicationCommandsJSONBody,
} from "discord-api-types/v9";
import {
  CommandInteraction,
  ModalSubmitInteraction,
  Permissions,
} from "discord.js";
import Context from "../context";

/**
 * Response of a command check, a message will only exist on pass = false
 */
export type CheckResponse =
  | {
      pass: true;
    }
  | {
      pass: false;
      message: string;
    };

export interface SlashCommand {
  /**
   * Data for command, e.g. the name, description, options
   */
  command:
    | RESTPostAPIChatInputApplicationCommandsJSONBody
    | RESTPostAPIApplicationCommandsJSONBody;
  /**
   * Required permissions for the **bot** to run the command, ie. ban members
   */
  requiredBotPermissions?: Permissions;
  /**
   * Required permissions for the **user** to run the command
   */
  requiredUserPermissions?: Permissions;
  /**
   * Check function that will run before a command to see if it should be run.
   * Return true to allow the command, or a string with an error to show the user.
   */
  check?: (
    ctx: Context,
    interaction: CommandInteraction
  ) => Promise<CheckResponse>;
  /**
   * Command handler function
   */
  handler: (ctx: Context, interaction: CommandInteraction) => Promise<void>;
  /**
   * Modal submit handler function
   */
  modalSubmitHandler?: (
    ctx: Context,
    interaction: ModalSubmitInteraction
  ) => Promise<void>;
}
