import * as z from "zod"

// Helper schema for JSON fields
type Literal = boolean | number | string
type Json = Literal | { [key: string]: Json } | Json[]
const literalSchema = z.union([z.string(), z.number(), z.boolean()])
const jsonSchema: z.ZodSchema<Json> = z.lazy(() => z.union([literalSchema, z.array(jsonSchema), z.record(jsonSchema)]))

export const UserModel = z.object({
  id: z.bigint(),
  isPatron: z.boolean(),
  patronEmoji: z.string().nullish(),
  rep: z.bigint(),
  fishies: z.bigint(),
  lastRep: z.date().nullish(),
  lastFishies: z.date().nullish(),
  lastfmUsername: z.string().nullish(),
  profileData: jsonSchema,
})
