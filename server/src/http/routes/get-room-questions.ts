import { type FastifyPluginAsyncZod } from "fastify-type-provider-zod";
import { desc, eq } from "drizzle-orm";
import { schema } from "../../db/schema/index.ts";
import { db } from "../../db/connection.ts";
import z from "zod";

export const getRoomQuestions: FastifyPluginAsyncZod = async (app) => {
  app.get(
    "/rooms/:roomId/questions",
    {
      schema: {
        params: z.object({
          roomId: z.string(),
        }),
      },
    },
    async (req) => {
      const { roomId } = req.params;

      const result = await db
        .select({
          id: schema.questions.id,
          question: schema.questions.question,
          answer: schema.questions.answer,
          createdAt: schema.questions.createdAt,
        })
        .from(schema.questions)
        .where(eq(schema.questions.roomId, roomId))
        .orderBy(desc(schema.questions.createdAt));

      return result;
    }
  );
};
