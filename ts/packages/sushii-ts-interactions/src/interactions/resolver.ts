/**
 * Derived from discordjs/discord.js
 *
 * https://github.com/discordjs/discord.js/blob/stable/src/structures/CommandInteractionOptionResolver.js#L8
 *
 * License: https://github.com/discordjs/discord.js/blob/main/packages/discord.js/LICENSE
 */

import {
  APIApplicationCommandInteractionDataOption,
  APIChatInputApplicationCommandInteractionDataResolved,
  ApplicationCommandOptionType,
  APIApplicationCommandInteractionDataBasicOption,
  APIUser,
  APIAttachment,
  APIRole,
  APIInteractionDataResolvedChannel,
  APIInteractionDataResolvedGuildMember,
} from "discord-api-types/v9";

function isDataBasicOption(
  option: APIApplicationCommandInteractionDataOption
): option is APIApplicationCommandInteractionDataBasicOption {
  return (
    option.type !== ApplicationCommandOptionType.Subcommand &&
    option.type !== ApplicationCommandOptionType.SubcommandGroup
  );
}

type OptionValue<T extends ApplicationCommandOptionType> =
  T extends ApplicationCommandOptionType.Boolean
    ? boolean
    : T extends ApplicationCommandOptionType.Channel
    ? APIInteractionDataResolvedChannel
    : T extends
        | ApplicationCommandOptionType.Integer
        | ApplicationCommandOptionType.Number
    ? number
    : T extends ApplicationCommandOptionType.Role
    ? APIRole
    : T extends ApplicationCommandOptionType.User
    ? APIUser
    : T extends ApplicationCommandOptionType.String
    ? string
    : T extends ApplicationCommandOptionType.Mentionable
    ? APIUser | APIRole | APIInteractionDataResolvedGuildMember
    : T extends ApplicationCommandOptionType.Attachment
    ? APIAttachment
    : never;

/**
 * A resolver for command interaction options.
 */
export default class CommandInteractionOptionResolver {
  private readonly group: string | null;

  private readonly subcommand: string | null;

  private readonly hoistedOptions: APIApplicationCommandInteractionDataOption[];

  private readonly resolved: APIChatInputApplicationCommandInteractionDataResolved;

  constructor(
    options: APIApplicationCommandInteractionDataOption[] = [],
    resolved: APIChatInputApplicationCommandInteractionDataResolved = {}
  ) {
    /**
     * The name of the subcommand group.
     * @type {?string}
     * @private
     */
    this.group = null;

    /**
     * The name of the subcommand.
     * @type {?string}
     * @private
     */
    this.subcommand = null;

    /**
     * The bottom-level options for the interaction.
     * If there is a subcommand (or subcommand and group), this is the options for the subcommand.
     * @type {CommandInteractionOption[]}
     * @private
     */
    this.hoistedOptions = options;

    // Hoist subcommand group if present
    if (
      this.hoistedOptions[0]?.type ===
      ApplicationCommandOptionType.SubcommandGroup
    ) {
      this.group = this.hoistedOptions[0].name;
      this.hoistedOptions = this.hoistedOptions[0].options ?? [];
    }
    // Hoist subcommand if present
    if (
      this.hoistedOptions[0]?.type === ApplicationCommandOptionType.Subcommand
    ) {
      this.subcommand = this.hoistedOptions[0].name;
      this.hoistedOptions = this.hoistedOptions[0].options ?? [];
    }

    this.resolved = resolved;
  }

  /**
   * Gets an option by its name.
   * @param {string} name The name of the option.
   * @returns {?APIApplicationCommandInteractionDataOption} The option, if found.
   */
  get(name: string): APIApplicationCommandInteractionDataOption | undefined {
    return this.hoistedOptions.find((opt) => opt.name === name);
  }

  /**
   * Gets an option by name and property and checks its type.
   * @param {string} name The name of the option.
   * @param {ApplicationCommandOptionType} type The type of the option.
   * @param {string[]} properties The properties to check for for `required`.
   * @param {boolean} required Whether to throw an error if the option is not found.
   * @returns {?CommandInteractionOption} The option, if found.
   * @private
   */
  private getTypedOptionValue<T extends ApplicationCommandOptionType>(
    name: string,
    type: T
  ): OptionValue<T> | undefined {
    const option = this.get(name);
    if (!option) {
      return;
    }

    if (!isDataBasicOption(option)) {
      throw new TypeError("Not a basic option");
    }

    if (option.type !== type) {
      throw new TypeError("Option does not match input type");
    }

    // eslint-disable-next-line default-case
    switch (option.type) {
      case ApplicationCommandOptionType.User:
        return this.resolved.users?.[option.value] as OptionValue<T>;
      case ApplicationCommandOptionType.Role:
        return this.resolved.roles?.[option.value] as OptionValue<T>;
      case ApplicationCommandOptionType.Channel:
        return this.resolved.channels?.[option.value] as OptionValue<T>;
      case ApplicationCommandOptionType.Mentionable:
        return (this.resolved.members?.[option.value] ||
          this.resolved.users?.[option.value] ||
          this.resolved.roles?.[option.value]) as OptionValue<T>;
      case ApplicationCommandOptionType.Attachment:
        return this.resolved.attachments?.[option.value] as OptionValue<T>;
    }

    return option.value as OptionValue<T>;
  }

  /**
   * Gets the selected subcommand.
   * @param {boolean} [required=true] Whether to throw an error if there is no subcommand.
   * @returns The name of the selected subcommand, or null if not set and not required.
   */
  getSubcommand(): string | null {
    return this.subcommand;
  }

  /**
   * Gets the selected subcommand group.
   * @param {boolean} [required=true] Whether to throw an error if there is no subcommand group.
   * @returns The name of the selected subcommand group, or null if not set and not required.
   */
  getSubcommandGroup(): string | null {
    return this.group;
  }

  /**
   * Gets a boolean option.
   * @param {string} name The name of the option.
   * @returns The value of the option, or null if not set and not required.
   */
  getBoolean(name: string): boolean | undefined {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.Boolean);
  }

  /**
   * Gets a channel option.
   * @param {string} name The name of the option.
   * @returns The value of the option, or null if not set and not required.
   */
  getChannel(name: string): APIInteractionDataResolvedChannel | undefined {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.Channel);
  }

  /**
   * Gets a string option.
   * @param {string} name The name of the option.
   * @returns The value of the option, or null if not set and not required.
   */
  getString(name: string): string | undefined {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.String);
  }

  /**
   * Gets an integer option.
   * @param {string} name The name of the option.
   * @returns The value of the option, or null if not set and not required.
   */
  getInteger(name: string): number | undefined {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.Integer);
  }

  /**
   * Gets a number option.
   * @param {string} name The name of the option.
   * @returns The value of the option, or null if not set and not required.
   */
  getNumber(name: string): number | undefined {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.Number);
  }

  /**
   * Gets a user option.
   * @param {string} name The name of the option.
   * @returns The value of the option, or null if not set and not required.
   */
  getUser(name: string): APIUser | undefined {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.User);
  }

  /**
   * Gets a member option.
   * @param name The name of the option.
   * @returns The value of the option, or null if not set and not required.
   */
  getMember(name: string): APIInteractionDataResolvedGuildMember | undefined {
    const user = this.getTypedOptionValue(
      name,
      ApplicationCommandOptionType.User
    );

    if (!user) {
      return;
    }

    return this.resolved.members?.[user.id];
  }

  /**
   * Gets a role option.
   * @param name The name of the option.
   * @returns The value of the option, or null if not set and not required.
   */
  getRole(name: string): APIRole | undefined {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.Role);
  }

  /**
   * Gets a mentionable option.
   * @param  name The name of the option.
   * @returns
   * The value of the option, or null if not set and not required.
   */
  getMentionable(
    name: string
  ): APIUser | APIRole | APIInteractionDataResolvedGuildMember | undefined {
    return this.getTypedOptionValue(
      name,
      ApplicationCommandOptionType.Mentionable
    );
  }

  /**
   * Gets a message option.
   * @param {string} name The name of the option.
   * @returns {?(Message|APIMessage)}
   * The value of the option, or null if not set and not required.
   */
  // TODO: support message interactions
  // getMessage(name: string) {
  //   return this.getTypedOptionValue(name, "_MESSAGE");
  // }

  /**
   * Gets the focused option.
   *
   * @returns {APIApplicationCommandInteractionDataOption }
   * The value of the option, or the whole option if getFull is true
   */
  getFocusedOption(): APIApplicationCommandInteractionDataOption | undefined {
    return this.hoistedOptions.find(
      (option) =>
        option.type === ApplicationCommandOptionType.String ||
        (option.type === ApplicationCommandOptionType.Number && option.focused)
    );
  }
}
