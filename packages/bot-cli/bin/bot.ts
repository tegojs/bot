#!/usr/bin/env node

/**
 * CLI entry point
 */

import { program } from "../src/index.js";

program.parse(process.argv);
