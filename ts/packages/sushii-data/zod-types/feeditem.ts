import * as z from "zod"

export const FeedItemModel = z.object({
  feedId: z.string(),
  itemId: z.string(),
})
