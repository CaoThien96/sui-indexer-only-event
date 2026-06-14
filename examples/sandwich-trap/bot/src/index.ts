import { config as loadEnv } from 'dotenv';
import { resolve } from 'node:path';
import { SuiClient } from '@mysten/sui/client';
import { prepareWalletCoins } from './coin-merge.js';
import { loadConfig } from './config.js';
import {
  getBaitKeypair,
  getDumpKeypair,
  getSingleKeypair,
} from './keypair.js';
import { readPoolState } from './pool-reader.js';
import {
  executeBait,
  executeDump,
  executeTrapRound,
} from './trap-executor.js';
import {
  createShutdownSignal,
  parseMaxRounds,
  runTrapLoop,
} from './trap-loop.js';

loadEnv({ path: resolve(process.cwd(), '.env') });
loadEnv({ path: resolve(process.cwd(), '../.env') });

function jsonReplacer(_key: string, value: unknown) {
  return typeof value === 'bigint' ? value.toString() : value;
}

function resolveKeypairs(config: ReturnType<typeof loadConfig>) {
  if (config.trapMode === 'single-tx') {
    return { single: getSingleKeypair() };
  }
  return { bait: getBaitKeypair(), dump: getDumpKeypair() };
}

async function main() {
  const cmd = process.argv[2] ?? 'trap';
  const configArg =
    process.argv.find((a) => a.startsWith('--config='))?.split('=')[1]
    ?? '../config/mainnet.json';
  const configPath = resolve(process.cwd(), configArg);
  const config = loadConfig(configPath);
  const client = new SuiClient({ url: config.rpcUrl });

  if (cmd === 'read-pool') {
    const pool = await readPoolState(client, config);
    console.log(JSON.stringify({ pool, trapMode: config.trapMode }, jsonReplacer, 2));
    return;
  }

  const keypairs = resolveKeypairs(config);

  if (cmd === 'merge-coins') {
    if (config.trapMode === 'parallel-tx') {
      const [bait, dump] = await Promise.all([
        prepareWalletCoins(client, keypairs.bait!, config, 'bait'),
        prepareWalletCoins(client, keypairs.dump!, config, 'dump'),
      ]);
      console.log(JSON.stringify({ bait, dump }, null, 2));
    } else {
      const single = await prepareWalletCoins(client, keypairs.single!, config, 'trap');
      console.log(JSON.stringify({ single }, null, 2));
    }
    return;
  }

  if (cmd === 'bait') {
    const kp = config.trapMode === 'parallel-tx' ? keypairs.bait! : keypairs.single!;
    const result = await executeBait(client, kp, config);
    console.log(JSON.stringify(result, null, 2));
    return;
  }

  if (cmd === 'dump') {
    const kp = config.trapMode === 'parallel-tx' ? keypairs.dump! : keypairs.single!;
    const results = await executeDump(client, kp, config);
    console.log(JSON.stringify(results, null, 2));
    return;
  }

  if (cmd === 'trap') {
    const result = await executeTrapRound(client, keypairs, config);
    console.log(JSON.stringify(result, null, 2));
    return;
  }

  if (cmd === 'trap-loop') {
    await runTrapLoop(client, keypairs, config, {
      maxRounds: parseMaxRounds(process.argv),
      signal: createShutdownSignal(),
    });
    return;
  }

  console.error(
    `unknown command: ${cmd} (try: read-pool | merge-coins | bait | dump | trap | trap-loop)`,
  );
  process.exit(1);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
