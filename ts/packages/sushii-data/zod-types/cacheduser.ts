import * as z from "zod"

export const CachedUserModel = z.object({
  id: z.bigint(),
  avatarUrl: z.string(),
  name: z.string(),
  discriminator: z.number().int(),
  lastChecked: z.date(),
})
