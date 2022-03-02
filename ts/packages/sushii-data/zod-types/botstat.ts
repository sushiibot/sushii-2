import * as z from "zod"

export const BotStatModel = z.object({
  name: z.string(),
  category: z.string(),
  count: z.bigint(),
  createdAt: z.date(),
  updatedAt: z.date(),
})
