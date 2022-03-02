import * as z from "zod"
import { CompleteFeedSubscription, RelatedFeedSubscriptionModel } from "./index"

// Helper schema for JSON fields
type Literal = boolean | number | string
type Json = Literal | { [key: string]: Json } | Json[]
const literalSchema = z.union([z.string(), z.number(), z.boolean()])
const jsonSchema: z.ZodSchema<Json> = z.lazy(() => z.union([literalSchema, z.array(jsonSchema), z.record(jsonSchema)]))

export const FeedModel = z.object({
  feedId: z.string(),
  metadata: jsonSchema,
})

export interface CompleteFeed extends z.infer<typeof FeedModel> {
  feedSubscriptions: CompleteFeedSubscription[]
}

/**
 * RelatedFeedModel contains all relations on your model in addition to the scalars
 *
 * NOTE: Lazy required in case of potential circular dependencies within schema
 */
export const RelatedFeedModel: z.ZodSchema<CompleteFeed> = z.lazy(() => FeedModel.extend({
  feedSubscriptions: RelatedFeedSubscriptionModel.array(),
}))
