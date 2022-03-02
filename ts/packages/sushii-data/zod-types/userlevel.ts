import * as z from "zod"

export const UserLevelModel = z.object({
  userId: z.bigint(),
  guildId: z.bigint(),
  msgAllTime: z.bigint(),
  msgMonth: z.bigint(),
  msgWeek: z.bigint(),
  msgDay: z.bigint(),
  lastMsg: z.date(),
})
