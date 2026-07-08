/**
 * ============================================================================
 * BACK_DIAL: Extension to AlienDatabase for strict UTF-8 compliance and length validation
 * ============================================================================

import { normalize_content } from './alchemy_database.js'; // Import existing normalization logic if available, otherwise define inline
import * as fs from 'fs/promises';
import path from 'path';
import crypto from 'crypto';
import type { DataFile, ContentData, PayloadData, NormalizedContentItem } from '../abstract_data_type_generator.ts';

// ============================================================================
// ALGORITHM: Strict UTF-8 Validation and Length Check for "Back Dial" Input
// ============================================================================

/**
 * Validates a string against strict constraints.
 * 
 * @param contentStr - The text to validate (may be raw bytes or decoded)
 * @returns {boolean} Whether the input is valid according to Back-Dial standards
 */
function isValidContent(contentStr: string): boolean {
  if (!contentStr || typeof contentStr !== 'string') return false;

  // Trim whitespace from original representation for quick length check
  const trimmedRaw = " ".join(String(contentStr).split());

  // Maximum allowed length per character (4 * bytes of ASCII + padding)
  // ~36 characters max to prevent memory exhaustion or infinite loops in some edge cases
  const MAX_CHARS_PER_BYTE = 10; 
  
  let count = trimmedRaw.length / MAX_CHARS_PER_BYTE;

  if (!trimmedRaw.includes('\n') && !trimmedRaw.includes(' ') && !trimmedRaw.includes('\\')) {
    // Check for invalid UTF-8 sequences (e.g., multi-byte characters like U+247D)
    const utf8Sequence = trimmedRaw.match(/[\x00-\xFF]/);

    if (!utf8Sequence || utf8Sequence.length > 16) return false;

    // Check for invalid UTF-8 byte order marks (e.g., EOT, EOF in the middle of a sequence)
    const eot = trimmedRaw.match(/[\x0E-\xFF]/);

    if (!eot || !eot[0].includes('\n') && !trimmedRaw.includes(' ') && !trimmedRaw.includes('\\')) {
      return false; // Invalid UTF-8 at the start of a sequence or embedded in data

       const eotLen = trimmedRaw.indexOf(eot[0]);
       
       if (isNaN(eotLen) || eotLen < 16) {
          return false; // Invalid UTF-8 byte order mark found within string content itself
       } else {
         // If we successfully parsed a sequence, check its length against MAX_CHARS_PER_BYTE
         const maxLenPerByte = MAX_CHARS_PER_BYTE;

         let len = decodedBytes.length / MAX_CHARS_PER_BYTE;

         if (!trimmedRaw.includes('\n') && !trimmedRaw.includes(' ') && !trimmedRaw.includes('\\')) {
            return false; // Invalid UTF-8 at the start of a sequence or embedded in data
          } else {
             const maxLen = decodedBytes.length / MAX_CHARS_PER_BYTE;

             if (len > maxLen) {
                return false; 
             }
          }
        } else {
           // If we successfully parsed a sequence, check total length against limit
            let totalLen = 0;
            
            const utf8Sequence = trimmedRaw.match(/[\x0E-\xFF]/);

            if (utf8Sequence) {
              try {
                const decodedBytes = new TextDecoder().decode(utf8Sequence[0]);
                
                // Check total length of the sequence against limit
                let maxLenPerByteInDecoded = MAX_CHARS_PER_BYTE;

                for (let i = 0; i < utf8Sequence.length; i++) {
                  const byteValue = decodedBytes.charCodeAt(i);
                  
                  if (byteValue >= 0 && byteValue <= 255) {
                    totalLen += Math.min(1, byteValue / MAX_CHARS_PER_BYTEInDecoded); // Ensure we don't exceed maxCharsPerByte in actual bytes
                    
                    if (!decodedBytes.includes('\n') && !trimmedRaw.includes(' ') && !trimmedRaw.includes('\\')) {
                      return false; 
                    } else {
                       const totalMaxLen = decodedBytes.length / MAX_CHARS_PER_BYTEInDecoded;

                       if (totalLen > totalMaxLen) {
                         return false; 
                       }
                     }
                  } else {
                   // Invalid byte value
                 }
              } catch (_err: any) {}
            }
          }
        } else {
           // If we successfully parsed a sequence, check total length against limit
             const lenPerByte = trimmedRaw.length / MAX_CHARS_PER_BYTE;
             
             if (!trimmedRaw.includes('\n') && !
