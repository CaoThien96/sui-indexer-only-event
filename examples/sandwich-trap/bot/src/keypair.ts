import { readFileSync } from 'node:fs';
import { decodeSuiPrivateKey } from '@mysten/sui/cryptography';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';

export function keypairFromSecret(secret: string): Ed25519Keypair {
  const trimmed = secret.trim();
  if (trimmed.startsWith('suiprivkey')) {
    const { secretKey } = decodeSuiPrivateKey(trimmed);
    return Ed25519Keypair.fromSecretKey(secretKey);
  }
  return Ed25519Keypair.fromSecretKey(trimmed);
}

export function getSingleKeypair(): Ed25519Keypair {
  const secret = process.env.SUI_SECRET_KEY;
  if (!secret) {
    throw new Error('SUI_SECRET_KEY is required for single-tx mode');
  }
  return keypairFromSecret(secret);
}

export function getBaitKeypair(): Ed25519Keypair {
  const secret = process.env.SUI_SECRET_KEY_BAIT ?? process.env.SUI_SECRET_KEY;
  if (!secret) {
    throw new Error('SUI_SECRET_KEY_BAIT (or SUI_SECRET_KEY) is required for bait wallet');
  }
  return keypairFromSecret(secret);
}

export function getDumpKeypair(): Ed25519Keypair {
  const secret = process.env.SUI_SECRET_KEY_DUMP;
  if (!secret) {
    throw new Error('SUI_SECRET_KEY_DUMP is required for dump wallet in parallel-tx mode');
  }
  return keypairFromSecret(secret);
}

export function getKeypairFromKeystore(): Ed25519Keypair {
  const keystorePath = `${process.env.HOME}/.sui/sui_config/sui.keystore`;
  const entries = JSON.parse(readFileSync(keystorePath, 'utf8')) as string[];
  return keypairFromSecret(entries[0]!);
}
