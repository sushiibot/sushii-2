import {
  APIApplicationCommandInteractionDataOption,
  APIChatInputApplicationCommandInteractionDataResolved,
  ApplicationCommandOptionType,
  APIApplicationCommandInteractionDataBasicOption,
  APIUser,
  APIAttachment,
  APIChannel,
  APIRole,
  APIGuildMember,
  APIInteractionDataResolvedChannel,
  APIInteractionDataResolvedGuildMember,
} from "discord-api-types/v9";

/**
 * A resolver for command interaction options.
 */
export default class CommandInteractionOptionResolver {
  private group: string | null;

  private subcommand: string | null;

  private hoistedOptions: APIApplicationCommandInteractionDataOption[];

  private resolved: APIChatInputApplicationCommandInteractionDataResolved;

  constructor(
    options: APIApplicationCommandInteractionDataOption[],
    resolved: APIChatInputApplicationCommandInteractionDataResolved
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

    /**
     * The interaction options array.
     * @name CommandInteractionOptionResolver#data
     * @type {ReadonlyArray<CommandInteractionOption>}
     * @readonly
     */
    Object.defineProperty(this, "data", { value: Object.freeze([...options]) });

    /**
     * The interaction resolved data
     * @name CommandInteractionOptionResolver#resolved
     * @type {Readonly<CommandInteractionResolvedData>}
     */
    Object.defineProperty(this, "resolved", { value: Object.freeze(resolved) });

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

  private getTypedOptionValue(
    name: string,
    type: ApplicationCommandOptionType.Boolean
  ): boolean | undefined;
  private getTypedOptionValue(
    name: string,
    type: ApplicationCommandOptionType.User
  ): APIUser | undefined;
  private getTypedOptionValue(
    name: string,
    type: ApplicationCommandOptionType.Attachment
  ): APIAttachment | undefined;
  private getTypedOptionValue(
    name: string,
    type: ApplicationCommandOptionType.Channel
  ): APIInteractionDataResolvedChannel | undefined;
  private getTypedOptionValue(
    name: string,
    type: ApplicationCommandOptionType.Mentionable
  ): APIUser | APIRole | APIInteractionDataResolvedGuildMember | undefined;
  private getTypedOptionValue(
    name: string,
    type: ApplicationCommandOptionType.Role
  ): APIRole | undefined;
  private getTypedOptionValue(
    name: string,
    type: ApplicationCommandOptionType.String
  ): string | undefined;
  private getTypedOptionValue(
    name: string,
    type:
      | ApplicationCommandOptionType.Integer
      | ApplicationCommandOptionType.Number
  ): number | undefined;

  /**
   * Gets an option by name and property and checks its type.
   * @param {string} name The name of the option.
   * @param {ApplicationCommandOptionType} type The type of the option.
   * @param {string[]} properties The properties to check for for `required`.
   * @param {boolean} required Whether to throw an error if the option is not found.
   * @returns {?CommandInteractionOption} The option, if found.
   * @private
   */
  private getTypedOptionValue(
    name: string,
    type: ApplicationCommandOptionType
  ) {
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

    switch (option.type) {
      case ApplicationCommandOptionType.User:
        return this.resolved.users?.[option.value];
      case ApplicationCommandOptionType.Role:
        return this.resolved.roles?.[option.value];
      case ApplicationCommandOptionType.Channel:
        return this.resolved.channels?.[option.value];
      case ApplicationCommandOptionType.Mentionable:
        return (
          this.resolved.members?.[option.value] ||
          this.resolved.users?.[option.value] ||
          this.resolved.roles?.[option.value]
        );
      case ApplicationCommandOptionType.Attachment:
        return this.resolved.attachments?.[option.value];
    }

    return option.value;
  }

  /**
   * Gets the selected subcommand.
   * @param {boolean} [required=true] Whether to throw an error if there is no subcommand.
   * @returns {?string} The name of the selected subcommand, or null if not set and not required.
   */
  getSubcommand(required = true) {
    if (required && !this.subcommand) {
      throw new TypeError("COMMAND_INTERACTION_OPTION_NO_SUB_COMMAND");
    }
    return this.subcommand;
  }

  /**
   * Gets the selected subcommand group.
   * @param {boolean} [required=true] Whether to throw an error if there is no subcommand group.
   * @returns {?string} The name of the selected subcommand group, or null if not set and not required.
   */
  getSubcommandGroup(required = true) {
    if (required && !this.group) {
      throw new TypeError("COMMAND_INTERACTION_OPTION_NO_SUB_COMMANDgroup");
    }
    return this.group;
  }

  /**
   * Gets a boolean option.
   * @param {string} name The name of the option.
   * @returns {?boolean} The value of the option, or null if not set and not required.
   */
  getBoolean(name: string): boolean | undefined {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.Boolean);
  }

  /**
   * Gets a channel option.
   * @param {string} name The name of the option.
   * @returns {?(GuildChannel|ThreadChannel|APIChannel)}
   * The value of the option, or null if not set and not required.
   */
  getChannel(name: string) {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.Channel);
  }

  /**
   * Gets a string option.
   * @param {string} name The name of the option.
   * @returns {?string} The value of the option, or null if not set and not required.
   */
  getString(name: string) {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.String);
  }

  /**
   * Gets an integer option.
   * @param {string} name The name of the option.
   * @returns {?number} The value of the option, or null if not set and not required.
   */
  getInteger(name: string) {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.Integer);
  }

  /**
   * Gets a number option.
   * @param {string} name The name of the option.
   * @returns {?number} The value of the option, or null if not set and not required.
   */
  getNumber(name: string) {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.Number);
  }

  /**
   * Gets a user option.
   * @param {string} name The name of the option.
   * @returns {?User} The value of the option, or null if not set and not required.
   */
  getUser(name: string): APIUser | undefined {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.User);
  }

  /**
   * Gets a member option.
   * @param {string} name The name of the option.
   * @returns {?(GuildMember|APIGuildMember)}
   * The value of the option, or null if not set and not required.
   */
  getMember(name: string) {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.User);
  }

  /**
   * Gets a role option.
   * @param {string} name The name of the option.
   * @returns {?(Role|APIRole)} The value of the option, or null if not set and not required.
   */
  getRole(name: string) {
    return this.getTypedOptionValue(name, ApplicationCommandOptionType.Role);
  }

  /**
   * Gets a mentionable option.
   * @param {string} name The name of the option.
   * @returns {?(User|GuildMember|APIGuildMember|Role|APIRole)}
   * The value of the option, or null if not set and not required.
   */
  getMentionable(name: string) {
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
  getMessage(name: string) {
    return this.getTypedOptionValue(name, "_MESSAGE");
  }

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

function isDataBasicOption(
  option: APIApplicationCommandInteractionDataOption
): option is APIApplicationCommandInteractionDataBasicOption {
  return (
    option.type !== ApplicationCommandOptionType.Subcommand &&
    option.type !== ApplicationCommandOptionType.SubcommandGroup
  );
}
