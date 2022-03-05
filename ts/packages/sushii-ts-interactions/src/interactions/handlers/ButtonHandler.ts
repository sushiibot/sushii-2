import { ButtonInteraction } from "discord.js";
import Context from "../../context";

export default abstract class ButtonHandler {
  abstract readonly buttonId: string;
  /**
   * Button submit handler function
   */
  abstract handleButton(
    ctx: Context,
    interaction: ButtonInteraction
  ): Promise<void>;
}
