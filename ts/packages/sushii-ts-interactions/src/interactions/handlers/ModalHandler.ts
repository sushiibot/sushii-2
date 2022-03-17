import { APIModalSubmitInteraction } from "discord-api-types/v9";
import Context from "../../context";

export default abstract class ModalHandler {
  abstract readonly modalId: string;

  /**
   * Modal submit handler function
   */
  abstract handleModalSubmit: (
    ctx: Context,
    interaction: APIModalSubmitInteraction
  ) => Promise<void>;
}
