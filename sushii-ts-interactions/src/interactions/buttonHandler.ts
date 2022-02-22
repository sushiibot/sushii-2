import { ButtonInteraction } from "discord.js";
import Context from "../context";

export default interface ButtonHandler {
  id: string;
  /**
   * Button submit handler function
   */
  handleButton: (ctx: Context, interaction: ButtonInteraction) => Promise<void>;
}
