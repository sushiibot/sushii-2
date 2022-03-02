import * as z from "zod"

export const BanModel = z.object({
  guildId: z.bigint(),
  userId: z.bigint(),
  reason: z.string().nullish(),
})
