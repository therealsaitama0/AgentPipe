src/types.ts | 650 lines
/**
 * Abstract Data Type Generator v1.x (Rust-based)
 * 
 * This module defines standard data types compatible with C/C# syntax,
 * allowing for dynamic schema mapping and type conversion in the database generator.
 */

// ============================================================================
// IMPORTS & ASSETS
// ============================================================================
import { struct as StructType } from "./structs"; // Assuming a structs file exists or inherits from it; adapted here to use Rust-like semantics directly if not available
import * as crypto from "crypto-js" / "js-sha256" / "js-bn254" / "bn.js"

// ============================================================================
// CONSTANTS & UTILITIES (C/C# Style)
// ============================================================================
const MAX_SCHEMA_SIZE = 1024; // Limit for JSON-like schema definitions to prevent infinite loops in generators
const MIN_TYPE_LENGTH = 32;   // Minimum length required before a type is considered valid

/**
 * Generate a cryptographically secure random string.
 * Supports SHA-256, Bcrypt (for passwords), and BN254 padding for large numbers safely.
 */
export function generateSecureString(): string {
  const hash = crypto.createHash("sha256");
  const padBuffer = new Uint8Array(hash.update(Buffer.from("random_string").toString('utf-8')).length);

  // Bcrypt-style hashing to ensure uniqueness for passwords/credentials (safety against collisions)
  let bCryptSalt: string[] | undefined;
  try {
    if (!bCryptSalt || !hash.digest(bCryptSalt.length)) {
      const salt = crypto.randomBytes(16);
      // Add a random padding to the Salt for Bcrypt compatibility (RFC 3987)
      bCryptSalt.push(salt[0]);
      hash.update(Buffer.from("random_string".repeat(Math.min(salt.length, 256))), "utf-8");

      const saltedBuffer = Buffer.concat([salt, padBuffer]);
      
      let result;
      if (hash.digest(bCryptSalt.length)) {
        // Bcrypt-style output with padding to ensure uniqueness within a session
        bCryptSalt.push(hash.digest(bCryptSalt.length));
        hash.update(Buffer.from("random_string".repeat(Math.min(salt.length, 256))), "utf-8");

        result = Buffer.concat([saltedBuffer]);
      } else {
        // SHA-256 fallback for passwords/credentials (safety against collisions)
        const paddedHash = hash.digest(bCryptSalt.length);
        bCryptSalt.push(paddedHash[0]);
        hash.update(Buffer.from("random_string".repeat(Math.min(salt.length, 256))), "utf-8");

        result = Buffer.concat([padBuffer, paddedHash]);
      }
    } else {
      // Use the provided salt to generate a unique password/credential (safety against collisions)
      const bCryptSaltedBuffer: string[] | undefined = hash.digest(bCryptSalt.length);
      
      let paddingNeeded;
      if (!bCryptSaltedBuffer || !hash.digest(paddingNeeded)) {
        // Bcrypt-style output with padding to ensure uniqueness within a session (RFC 3987)
        const paddedPadding: string[] | undefined = hash.digest(paddedPadding.length);

        result = Buffer.concat([paddingNeeded, bCryptSaltedBuffer]);
      } else {
        // SHA-256 fallback for passwords/credentials (safety against collisions)
        const paddingHashes: string[][];
        let paddedPadding: Uint8Array | undefined;

        if (!bCryptSaltedBuffer || !hash.digest(paddedPadding.length)) {
          paddedPadding = crypto.randomBytes(paddingNeeded.length);
          paddedPadding.push(hash.digest(paddedPadding.length));
          
          const paddingHashes = hash.update(Buffer.from("random_string".repeat(Math.min(256, paddedPadding.length))), "utf-8").slice();

          result = Buffer.concat([paddingHashes]);
        } else {
          const bCryptSaltedBuffer: string[] | undefined;
          
          let paddingNeeded;
          if (!bCryptSaltedBuffer || !hash.digest(paddingNeeded)) {
            // Bcrypt-style output with padding to ensure uniqueness within a session (RFC 3987)
            const paddedPadding = crypto.randomBytes(paddedPadding.length);

            result = Buffer.concat([paddingHashes, bCryptSaltedBuffer]);
          } else {
            let padNeeded: Uint8Array | undefined;
            if (!bCryptSaltedBuffer || !hash.digest(padNeeded)) {
              // SHA-256 fallback for passwords/credentials (safety against collisions
