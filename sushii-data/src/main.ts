import { NestFactory } from '@nestjs/core';
import { MicroserviceOptions, Transport } from '@nestjs/microservices';
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

  await app.listen();
}
bootstrap();
