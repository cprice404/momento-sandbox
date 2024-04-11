// Import the Momento redis compatibility client.
import {MomentoRedisAdapter, NewIORedisWrapper, NewIORedisClusterWrapper} from '@gomomento-poc/node-ioredis-client';
import {CacheClient, Configurations, CredentialProvider} from '@gomomento/sdk';

// Instantiate Momento Adapter Directly
const Redis = new MomentoRedisAdapter(
  new CacheClient({
    configuration: Configurations.Laptop.v1(),
    credentialProvider: CredentialProvider.fromEnvironmentVariable({
      environmentVariableName: 'MOMENTO_AUTH_TOKEN',
    }),
    defaultTtlSeconds: 3600,
  }),
  'myMomentoCache'
);

// Or use Momento wrapper functions to provide a more drop in friendly replacement when trying to toggle between Momento and Redis.
// Use env vars to configure Momento. See Momento wrapper function configuration section.
// Initialize basic Redis client Momento wrapper
const redisClient = NewIORedisWrapper();

// Or initilize Momento as you would an `ioredis cluster client
const redisClusterClient = NewIORedisClusterWrapper([], {});
