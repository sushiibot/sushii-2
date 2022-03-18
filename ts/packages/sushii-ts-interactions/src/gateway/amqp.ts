import { AMQPClient, AMQPConsumer, AMQPMessage } from "@cloudamqp/amqp-client";
import { ConfigI } from "../config";

export default class AmqpGateway {
  consumer?: AMQPConsumer;

  constructor(private amqp: AMQPClient, private config: ConfigI) {}

  async connect(handler: (msg: AMQPMessage) => Promise<void>): Promise<void> {
    const conn = await this.amqp.connect();
    const channel = await conn.channel();
    const queue = await channel.queue(this.config.amqpQueueName);

    // Automatically acknowledge messages, as recovering from tasks isn't that
    // important -- detection of failed tasks would be much too slow and would
    // rather respond with an immediate error (ie. Discord's built in failed to
    // respond to interaction).
    this.consumer = await queue.subscribe({ noAck: true }, handler);
  }

  async stop(): Promise<void> {
    await this.consumer?.cancel();
    await this.amqp.close();
  }
}
