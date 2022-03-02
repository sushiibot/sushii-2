import * as z from "zod"
import { CompleteMute, RelatedMuteModel } from "./index"

export const ModLogModel = z.object({
  guildId: z.bigint(),
  caseId: z.bigint(),
  action: z.string(),
  actionTime: z.date(),
  pending: z.boolean(),
  userId: z.bigint(),
  userTag: z.string(),
  executorId: z.bigint().nullish(),
  reason: z.string().nullish(),
  msgId: z.bigint().nullish(),
  attachments: z.string().array(),
})

export interface CompleteModLog extends z.infer<typeof ModLogModel> {
  mutes: CompleteMute[]
}

/**
 * RelatedModLogModel contains all relations on your model in addition to the scalars
 *
 * NOTE: Lazy required in case of potential circular dependencies within schema
 */
export const RelatedModLogModel: z.ZodSchema<CompleteModLog> = z.lazy(() => ModLogModel.extend({
  mutes: RelatedMuteModel.array(),
}))
