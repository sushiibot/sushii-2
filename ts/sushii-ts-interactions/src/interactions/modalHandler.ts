import { ModalSubmitInteraction } from "discord.js";
import Context from "../context";

export default interface ModalHandler {
  id: string;
  /**
   * Modal submit handler function
   */
  handleSubmit: (
    ctx: Context,
    interaction: ModalSubmitInteraction
  ) => Promise<void>;
}
