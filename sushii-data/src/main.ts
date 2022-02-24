import { NestFactory } from '@nestjs/core';
import { MicroserviceOptions, Transport } from '@nestjs/microservices';
import { Logger } from 'nestjs-pino';
import { join } from 'path';
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.createMicroservice<MicroserviceOptions>(
    AppModule,
    {
      transport: Transport.GRPC,
      options: {
        package: 'sushii.guild.config',
        protoPath: join(__dirname, '..', 'proto/guild/config.proto'),
        loader: {
          defaults: false,
          longs: String,
          includeDirs: [join(__dirname, '..', 'proto')],
        },
      },
    },
  );

  app.useLogger(app.get(Logger));
  await app.listen();
}
bootstrap();
