import { type FastifyPluginAsyncZod } from "fastify-type-provider-zod";
import { z } from "zod/v4";
import { schema } from "../../db/schema/index.ts";
import { db } from "../../db/connection.ts";

export const createQuestionRoute: FastifyPluginAsyncZod = async (app) => {
  app.post(
    "/rooms/:roomId/questions",
    {
      schema: {
        params: z.object({
          roomId: z.string(),
        }),
        body: z.object({
          question: z.string().min(1),
        }),
      },
    },
    async (req, res) => {
      const { question } = req.body;
      const { roomId } = req.params;

      const result = await db
        .insert(schema.questions)
        .values({
          roomId,
          question,
        })
        .returning();

      const insertedQuestion = result[0];

      if (!insertedQuestion) {
        throw new Error("Failed to create new room");
      }

      return res.status(201).send({ questionId: insertedQuestion.id });
    }
  );
};
