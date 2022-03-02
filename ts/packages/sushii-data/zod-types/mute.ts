import * as z from "zod"
import { CompleteModLog, RelatedModLogModel } from "./index"

export const MuteModel = z.object({
  guildId: z.bigint(),
  userId: z.bigint(),
  startTime: z.date(),
  endTime: z.date().nullish(),
  pending: z.boolean(),
  caseId: z.bigint().nullish(),
})

export interface CompleteMute extends z.infer<typeof MuteModel> {
  modLogs?: CompleteModLog | null
}

/**
 * RelatedMuteModel contains all relations on your model in addition to the scalars
 *
 * NOTE: Lazy required in case of potential circular dependencies within schema
 */
export const RelatedMuteModel: z.ZodSchema<CompleteMute> = z.lazy(() => MuteModel.extend({
  modLogs: RelatedModLogModel.nullish(),
}))
