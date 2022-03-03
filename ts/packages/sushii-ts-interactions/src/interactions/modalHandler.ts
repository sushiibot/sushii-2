import { ModalSubmitInteraction } from "discord.js";
import Context from "../context";

export default abstract class ModalHandler {
  abstract readonly modalId: string;
  /**
   * Modal submit handler function
   */
  abstract handleModalSubmit: (
    ctx: Context,
    interaction: ModalSubmitInteraction
  ) => Promise<void>;
}
