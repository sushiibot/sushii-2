import { APIMessageComponentInteraction } from "discord-api-types/v9";
import Context from "../../context";

export default abstract class ButtonHandler {
  abstract readonly buttonId: string;
  /**
   * Button submit handler function
   */
  abstract handleButton(
    ctx: Context,
    interaction: APIMessageComponentInteraction
  ): Promise<void>;
}
