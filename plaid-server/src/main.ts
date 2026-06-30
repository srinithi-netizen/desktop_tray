import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  app.enableCors({
    origin: true, // reflects whatever origin made the request — fine for trusted LAN use
    methods: ['GET', 'POST'],
  });

  const port = process.env.PORT || 3001;
  await app.listen(port, '0.0.0.0'); // explicit, so it's not ambiguous
  console.log(`✅ Plaid server running on http://0.0.0.0:${port}`);
}
bootstrap();