import * as z from "zod"

// Helper schema for JSON fields
type Literal = boolean | number | string
type Json = Literal | { [key: string]: Json } | Json[]
const literalSchema = z.union([z.string(), z.number(), z.boolean()])
const jsonSchema: z.ZodSchema<Json> = z.lazy(() => z.union([literalSchema, z.array(jsonSchema), z.record(jsonSchema)]))

export const GuildFeedModel = z.object({
  guildId: z.bigint(),
  channelId: z.bigint(),
  mentionRole: z.bigint().nullish(),
  feedName: z.string(),
  feedSource: z.string(),
  feedHash: z.string(),
  feedMetadata: jsonSchema,
})
