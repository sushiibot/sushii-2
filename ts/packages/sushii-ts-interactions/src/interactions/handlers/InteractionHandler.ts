import { APIInteraction, Permissions } from "discord-api-types/v9";
import Context from "../../context";

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

export default abstract class InteractionHandler {
  /**
   * Required permissions for the **bot** to run the command, ie. ban members
   */
  readonly requiredBotPermissions?: Permissions;

  /**
   * Required permissions for the **user** to run the command
   */
  readonly requiredUserPermissions?: Permissions;

  /**
   * If the interaction should only be run in a server
   */
  readonly serverOnly: boolean = false;

  /**
   * Check function that will run before a command to see if it should be run.
   * By default, this always passes.
   */
  // eslint-disable-next-line class-methods-use-this
  async check(
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _ctx: Context,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _interaction: APIInteraction
  ): Promise<CheckResponse> {
    return { pass: true };
  }

  /**
   * Interaction handler
   */
  abstract handler(ctx: Context, interaction: APIInteraction): Promise<void>;
}
