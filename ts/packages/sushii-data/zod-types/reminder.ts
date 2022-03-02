import * as z from "zod"

export const ReminderModel = z.object({
  userId: z.bigint(),
  description: z.string(),
  setAt: z.date(),
  expireAt: z.date(),
})
