import {
  APIChatInputApplicationCommandDMInteraction,
  APIChatInputApplicationCommandGuildInteraction,
  APIChatInputApplicationCommandInteraction,
  APIInteraction,
  ApplicationCommandType,
  GatewayDispatchEvents,
  GatewayInteractionCreateDispatch,
  GatewayOpcodes,
  InteractionType,
} from "discord-api-types/v9";

export function isGuildInteraction(
  interaction: APIChatInputApplicationCommandInteraction
): interaction is APIChatInputApplicationCommandGuildInteraction {
  return (
    interaction.user === undefined &&
    interaction.guild_id !== undefined &&
    interaction.member !== undefined
  );
}

export function isDMInteraction(
  interaction: APIChatInputApplicationCommandInteraction
): interaction is APIChatInputApplicationCommandDMInteraction {
  return (
    interaction.user !== undefined &&
    interaction.guild_id === undefined &&
    interaction.member === undefined
  );
}

export function isGatewayInteractionCreateDispatch(
  msg: any
): msg is GatewayInteractionCreateDispatch {
  return (
    msg &&
    msg.op === GatewayOpcodes.Dispatch &&
    msg.t === GatewayDispatchEvents.InteractionCreate
  );
}

export function isAPIChatInputApplicationCommandInteraction(
  interaction: APIInteraction
): interaction is APIChatInputApplicationCommandInteraction {
  return (
    interaction.type === InteractionType.ApplicationCommand &&
    interaction.data.type === ApplicationCommandType.ChatInput
  );
}
