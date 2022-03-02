import * as z from "zod"

export const GuildBanModel = z.object({
  guildId: z.bigint(),
  userId: z.bigint(),
})
