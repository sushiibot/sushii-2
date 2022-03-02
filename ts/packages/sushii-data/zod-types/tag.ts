import * as z from "zod"

export const TagModel = z.object({
  ownerId: z.bigint(),
  guildId: z.bigint(),
  tagName: z.string(),
  content: z.string(),
  useCount: z.bigint(),
  created: z.date(),
})
