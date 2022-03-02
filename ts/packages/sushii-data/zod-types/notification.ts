import * as z from "zod"

export const NotificationModel = z.object({
  userId: z.bigint(),
  guildId: z.bigint(),
  keyword: z.string(),
})
