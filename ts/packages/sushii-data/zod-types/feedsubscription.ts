import * as z from "zod"
import { CompleteFeed, RelatedFeedModel } from "./index"

export const FeedSubscriptionModel = z.object({
  feedId: z.string(),
  guildId: z.bigint(),
  channelId: z.bigint(),
  mentionRole: z.bigint().nullish(),
})

export interface CompleteFeedSubscription extends z.infer<typeof FeedSubscriptionModel> {
  feeds: CompleteFeed
}

/**
 * RelatedFeedSubscriptionModel contains all relations on your model in addition to the scalars
 *
 * NOTE: Lazy required in case of potential circular dependencies within schema
 */
export const RelatedFeedSubscriptionModel: z.ZodSchema<CompleteFeedSubscription> = z.lazy(() => FeedSubscriptionModel.extend({
  feeds: RelatedFeedModel,
}))
