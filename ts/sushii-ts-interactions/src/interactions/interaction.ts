import { Interaction, Permissions } from "discord.js";
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

export default class InteractionHandler {
  /**
   * Required permissions for the **bot** to run the command, ie. ban members
   */
  requiredBotPermissions?: Permissions;

  /**
   * Required permissions for the **user** to run the command
   */
  requiredUserPermissions?: Permissions;

  /**
   * If the interaction should only be run in a server
   */
  serverOnly: boolean;

  /**
   * Check function that will run before a command to see if it should be run.
   */
  check: () => Promise<CheckResponse>;

  /**
   * Field for the actual handler function
   */
  handler: (ctx: Context, interaction: Interaction) => Promise<void>;

  constructor() {
    this.serverOnly = false;
    // Default to always pass check
    this.check = async () => ({ pass: true });
    this.handler = async () => {};
  }

  public setServerOnly(serverOnly: boolean): this {
    this.serverOnly = serverOnly;

    return this;
  }

  public setRequiredBotPermission(permissions: Permissions): this {
    this.requiredBotPermissions = permissions;

    return this;
  }

  public setRequiredUserPermission(permissions: Permissions): this {
    this.requiredUserPermissions = permissions;

    return this;
  }

  public setCheck(check: () => Promise<CheckResponse>): this {
    this.check = check;

    return this;
  }

  public setHandler(
    handler: (ctx: Context, interaction: Interaction) => Promise<void>
  ): this {
    this.handler = handler;

    return this;
  }
}
