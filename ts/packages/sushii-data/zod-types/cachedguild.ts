import * as z from "zod"

export const CachedGuildModel = z.object({
  id: z.bigint(),
  name: z.string(),
  icon: z.string().nullish(),
  splash: z.string().nullish(),
  banner: z.string().nullish(),
  features: z.string().array(),
  createdAt: z.date(),
  updatedAt: z.date(),
})
