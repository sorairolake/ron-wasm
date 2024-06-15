#!/usr/bin/env -S deno run --allow-read

// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

import * as io from "@std/io";

import * as command from "@cliffy/command";

import * as ron from "../pkg/ron_wasm.js";

import { VERSION } from "./version.ts";

const { args } = await new command.Command()
  .name("ron2json")
  .version(VERSION)
  .description("An example of converting a RON string to a JSON string.")
  .arguments("[FILE:file]")
  .parse();

const input = args[0] === undefined
  ? io.readAllSync(Deno.stdin)
  : Deno.readFileSync(args[0]);
const string = new TextDecoder().decode(input);
const obj = ron.parse(string);

const output = JSON.stringify(obj);
console.log(output);
